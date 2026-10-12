Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$ProjectRoot = Resolve-Path (Join-Path $PSScriptRoot "..")
$SourceRoots = @(
	(Join-Path $ProjectRoot "src"),
	(Join-Path $ProjectRoot "tests")
)

$Files = Get-ChildItem -Path $SourceRoots -Recurse -Filter *.cs -File

foreach ($File in $Files)
{
	$Lines = [System.IO.File]::ReadAllLines($File.FullName)
	$Result = [System.Collections.Generic.List[string]]::new()
	$Depth = 0
	$PendingEnum = $false
	$EnumDepth = -1

	foreach ($RawLine in $Lines)
	{
		$Line = $RawLine.TrimEnd()
		if ([string]::IsNullOrWhiteSpace($Line))
		{
			if ($EnumDepth -ge 0)
			{
				continue
			}

			if ($Depth -ge 3)
			{
				continue
			}

			if ($Result.Count -eq 0)
			{
				continue
			}

			if ($Result[$Result.Count - 1] -eq "")
			{
				continue
			}

			$Result.Add("")
			continue
		}

		$TrimmedLine = $Line.Trim()
		if ($TrimmedLine -match '^(public|internal|private|protected)?\s*(readonly\s+)?enum\s+\w+')
		{
			$PendingEnum = $true
		}

		$Result.Add($Line)
		$OpenCount = ([regex]::Matches($Line, "\{")).Count
		$CloseCount = ([regex]::Matches($Line, "\}")).Count

		if ($PendingEnum -and $OpenCount -gt 0)
		{
			$EnumDepth = $Depth + $OpenCount - $CloseCount
			$PendingEnum = $false
		}

		$Depth = [Math]::Max(0, $Depth + $OpenCount - $CloseCount)

		if ($EnumDepth -ge 0 -and $Depth -lt $EnumDepth)
		{
			$EnumDepth = -1
		}
	}

	[System.IO.File]::WriteAllText(
		$File.FullName,
		($Result -join "`n") + "`n",
		[System.Text.UTF8Encoding]::new($false)
	)
}

dotnet format (Join-Path $ProjectRoot "QarStreamingSharp.csproj") --include src tests
