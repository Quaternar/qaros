# Helpers shared by the CI scripts in this folder. Dot-source it: . "$PSScriptRoot/common.ps1"
# Written for PowerShell 7 (pwsh, what the workflows use) and Windows PowerShell 5.1.

$ErrorActionPreference = 'Stop'

# The qaros repository root, whatever the caller's working directory.
$QarosRoot = (Resolve-Path (Join-Path $PSScriptRoot '..')).Path

function Test-QarosWindows {
    return [Environment]::OSVersion.Platform -eq [PlatformID]::Win32NT
}

# Fails with a message naming the tool and how to get it.
function Assert-Tool {
    param([Parameter(Mandatory)][string]$Name, [Parameter(Mandatory)][string]$Hint)
    if (-not (Get-Command $Name -ErrorAction SilentlyContinue)) {
        throw "Missing tool '$Name': $Hint"
    }
}

# Runs a native command, echoes it, and fails on a non-zero exit code.
function Invoke-Native {
    param(
        [Parameter(Mandatory)][string]$FilePath,
        [string[]]$Arguments = @(),
        [string]$WorkingDirectory = $QarosRoot
    )
    Write-Host "[$WorkingDirectory] $FilePath $($Arguments -join ' ')" -ForegroundColor Cyan
    Push-Location $WorkingDirectory
    # Windows PowerShell turns a native tool's stderr (cargo's progress, for one) into errors when
    # the output is redirected; with 'Stop' the first line would abort the run. The exit code decides.
    $previousPreference = $ErrorActionPreference
    $ErrorActionPreference = 'Continue'
    try {
        & $FilePath @Arguments
        if ($LASTEXITCODE -ne 0) { throw "'$FilePath $($Arguments -join ' ')' failed (exit code $LASTEXITCODE)." }
    }
    finally {
        $ErrorActionPreference = $previousPreference
        Pop-Location
    }
}

# True when a native command exits 0; its output is discarded.
function Test-NativeCommand {
    param([Parameter(Mandatory)][string]$FilePath, [string[]]$Arguments = @())
    $previousPreference = $ErrorActionPreference
    $ErrorActionPreference = 'Continue'
    try {
        & $FilePath @Arguments *> $null
        return $LASTEXITCODE -eq 0
    }
    catch {
        return $false
    }
    finally {
        $ErrorActionPreference = $previousPreference
    }
}

# Sets environment variables for the duration of $Script, then restores them, so a script called
# from an interactive session leaves that session as it was.
function Invoke-WithEnvironment {
    param([Parameter(Mandatory)][hashtable]$Variables, [Parameter(Mandatory)][scriptblock]$Script)
    $saved = @{}
    foreach ($name in $Variables.Keys) {
        $saved[$name] = [Environment]::GetEnvironmentVariable($name)
        [Environment]::SetEnvironmentVariable($name, $Variables[$name])
    }
    try {
        & $Script
    }
    finally {
        foreach ($name in $saved.Keys) { [Environment]::SetEnvironmentVariable($name, $saved[$name]) }
    }
}
