<#
.SYNOPSIS
    The C# binding tests (.github/workflows/csharp-binding-tests.yml), runnable locally.

.DESCRIPTION
    Runs `dotnet test` on qar-streaming-sharp/tests/QarStreaming.Tests (Release, net8.0) against
    native SDK binaries: qar-streaming-c.dll, qar-streaming-c-ext.dll and every DLL they load.
    Where they come from, first match wins:
      -Release <tag>    downloads the QAROS-CApi-*-RelWithDebInfo.zip asset of that release of
                        -Repository with gh, as CI does (gh logged in, or GH_TOKEN set to a token
                        that can read those releases);
      QAROS_NATIVE_BIN  a folder holding them;
      neither           the binding's MSBuild props decide: a qaros-native.props above qaros (a
                        repository that embeds qaros, the Quaternar one), else package/bin.
    QAROS_HUB_TEST_EXT_BIN, when set, is the folder of qar-hub-test-ext.dll for the hub follow tests
    (only a Quaternar build has it). With -Release and without it, those tests are skipped.

    The tests drive Direct3D 11 and Vulkan: Windows with a GPU only.

.EXAMPLE
    ./ci/csharp-binding-tests.ps1 -Release dev-feature-qaros-local-enrollment
.EXAMPLE
    $env:QAROS_NATIVE_BIN = 'C:\sdk\bin'; ./ci/csharp-binding-tests.ps1 -Filter 'FullyQualifiedName~Follow'
#>
param(
    # A vX.Y.Z tag, or dev-<branch> for a branch build.
    [string]$Release = '',
    [string]$Repository = 'Quaternar/Quaternar',
    # Where a downloaded release is unpacked (emptied first).
    [string]$RuntimeDirectory = '',
    # Where the .trx results go.
    [string]$ResultsDirectory = '',
    # A dotnet test --filter expression.
    [string]$Filter = ''
)

. "$PSScriptRoot/common.ps1"

if (-not (Test-QarosWindows)) { throw 'The C# binding tests drive Direct3D 11 and Vulkan: they run on Windows only.' }
Assert-Tool dotnet 'install the .NET 8 SDK (or newer) from https://dot.net'
if (-not ((& dotnet --list-runtimes) -match '^Microsoft\.NETCore\.App 8\.')) {
    throw "Missing the .NET 8 runtime: the tests run on net8.0. Install it from https://dot.net."
}

$sharpDirectory = Join-Path $QarosRoot 'qar-streaming-sharp'
if (-not $ResultsDirectory) { $ResultsDirectory = Join-Path $QarosRoot 'build/test-results/csharp' }

$nativeBin = ''
if ($Release) {
    Assert-Tool gh 'install the GitHub CLI from https://cli.github.com'
    if (-not $env:GH_TOKEN -and -not (Test-NativeCommand gh @('auth', 'status'))) {
        throw "gh is not logged in: run 'gh auth login', or set GH_TOKEN to a token that can read $Repository releases."
    }
    if (-not $RuntimeDirectory) { $RuntimeDirectory = Join-Path $QarosRoot "build/qaros-capi-runtime/$Release" }
    if (Test-Path -LiteralPath $RuntimeDirectory) { Remove-Item -LiteralPath $RuntimeDirectory -Recurse -Force }
    New-Item -ItemType Directory -Path $RuntimeDirectory | Out-Null
    Invoke-Native gh @('release', 'download', $Release, '--repo', $Repository,
        '--pattern', 'QAROS-CApi-*-RelWithDebInfo.zip', '--dir', $RuntimeDirectory)
    $zip = Get-ChildItem -LiteralPath $RuntimeDirectory -Filter 'QAROS-CApi-*-RelWithDebInfo.zip' | Select-Object -First 1
    if (-not $zip) { throw "No QAROS-CApi ZIP in release $Release of $Repository." }
    Expand-Archive -LiteralPath $zip.FullName -DestinationPath $RuntimeDirectory
    # The ZIP holds one folder named after itself.
    $nativeBin = Join-Path $RuntimeDirectory (Join-Path $zip.BaseName 'bin')
    Write-Host "Testing against $($zip.Name)"
}
elseif ($env:QAROS_NATIVE_BIN) {
    $nativeBin = $env:QAROS_NATIVE_BIN
}
else {
    Write-Host 'QAROS_NATIVE_BIN is not set: the binding''s props pick the native folder (qaros-native.props of an embedding repository, else package/bin).'
}

if ($nativeBin) {
    foreach ($dll in 'qar-streaming-c.dll', 'qar-streaming-c-ext.dll') {
        if (-not (Test-Path -LiteralPath (Join-Path $nativeBin $dll))) { throw "$dll missing in $nativeBin." }
    }
}

$arguments = @('test', 'tests/QarStreaming.Tests/QarStreaming.Tests.csproj', '-c', 'Release', '-f', 'net8.0')
if ($nativeBin) { $arguments += "-p:QarStreamingNativeBinDir=$nativeBin" }
# A release has no qar-hub-test-ext; never mix in one from another build.
if ($Release -and -not $env:QAROS_HUB_TEST_EXT_BIN) { $arguments += '-p:QarSkipHubTestExt=true' }
if ($Filter) { $arguments += @('--filter', $Filter) }
$arguments += @('--logger', 'trx;LogFileName=csharp-binding-tests.trx', '--results-directory', $ResultsDirectory)

$environment = @{}
if ($nativeBin) { $environment['QAROS_NATIVE_BIN'] = $nativeBin }
Invoke-WithEnvironment $environment { Invoke-Native dotnet $arguments $sharpDirectory }
Write-Host "C# binding tests passed. Results: $ResultsDirectory" -ForegroundColor Green
