<#
.SYNOPSIS
    The Vulkan cube build (.github/workflows/vulkan-cube-build.yml), runnable locally.

.DESCRIPTION
    Builds the Vulkan cube source app as Developer Guide > Getting Started tells a new user: Visual
    Studio's own CMake and Ninja from a developer shell, no Vulkan SDK (vcpkg brings the Vulkan
    headers, loader and glslc), the two documented commands, from the repository root:
      cmake --preset x64-windows -DBUILD_VULKAN_EXAMPLE=ON -DVCPKG_MANIFEST_FEATURES=examples-vulkan
      cmake --build --preset x64-windows-release --target qar-vulkan-source
    then runs the built qar-vulkan-source.exe --help. Build only: running the cube needs an
    installed QAROS and a GPU.

    A Vulkan SDK on the host would hide a gap in the documented setup. By default the script
    removes VULKAN_SDK and every PATH entry holding glslc from its own environment for the build.
    -Strict (what CI passes) fails instead: there the runner itself must have neither.

    The build tree is build/x64-windows of this repository; the first build compiles the vcpkg
    dependencies and takes a while.

.EXAMPLE
    ./ci/vulkan-cube-build.ps1
#>
param(
    [switch]$Strict
)

. "$PSScriptRoot/common.ps1"

if (-not (Test-QarosWindows)) { throw 'The Vulkan cube build runs on Windows only (Visual Studio).' }

$vswhere = Join-Path ${env:ProgramFiles(x86)} 'Microsoft Visual Studio\Installer\vswhere.exe'
if (-not (Test-Path -LiteralPath $vswhere)) { throw "Missing tool 'vswhere' ($vswhere): install Visual Studio 2022 with the C++ workload." }
$vs = & $vswhere -latest -products * -property installationPath
if (-not $vs) { throw 'No Visual Studio found: install Visual Studio 2022 with the C++ workload.' }

$scrubbed = @{}
if ($env:VULKAN_SDK) {
    if ($Strict) { throw "VULKAN_SDK is set ($env:VULKAN_SDK): the build would not prove the documented setup." }
    $scrubbed['VULKAN_SDK'] = $null
}
$glslc = Get-Command glslc -ErrorAction SilentlyContinue
if ($glslc) {
    if ($Strict) { throw "glslc is on PATH ($($glslc.Source)): the build would not prove the documented setup." }
    $separator = [IO.Path]::PathSeparator
    $kept = $env:PATH -split [regex]::Escape($separator) |
        Where-Object { $_ -and -not (Test-Path -LiteralPath (Join-Path $_ 'glslc.exe')) }
    $scrubbed['PATH'] = $kept -join $separator
}
foreach ($name in $scrubbed.Keys) { Write-Host "Hiding the Vulkan SDK from the build: $name changed for this run." -ForegroundColor Yellow }

Invoke-WithEnvironment $scrubbed {
    # The developer shell changes this process's environment (PATH, INCLUDE, LIB, ...); restore it
    # afterwards so an interactive session is left as it was.
    $saved = [Environment]::GetEnvironmentVariables()
    try {
        # Developer PowerShell for VS, as the page says: Visual Studio's own CMake and Ninja.
        Import-Module (Join-Path $vs 'Common7\Tools\Microsoft.VisualStudio.DevShell.dll')
        Enter-VsDevShell -VsInstallPath $vs -SkipAutomaticLocation -DevCmdArguments '-arch=x64 -host_arch=x64' | Out-Null
        if (Get-Command glslc -ErrorAction SilentlyContinue) { throw 'glslc is on PATH inside the developer shell: the build would not prove the documented setup.' }

        Invoke-Native cmake @('--preset', 'x64-windows', '-DBUILD_VULKAN_EXAMPLE=ON', '-DVCPKG_MANIFEST_FEATURES=examples-vulkan')
        Invoke-Native cmake @('--build', '--preset', 'x64-windows-release', '--target', 'qar-vulkan-source')
    }
    finally {
        foreach ($name in @([Environment]::GetEnvironmentVariables().Keys)) {
            if (-not $saved.Contains($name)) { [Environment]::SetEnvironmentVariable($name, $null) }
        }
        foreach ($name in $saved.Keys) { [Environment]::SetEnvironmentVariable($name, $saved[$name]) }
    }
}

# The documented output exists and starts.
$exe = Join-Path $QarosRoot 'build\x64-windows\qar-streaming-c\examples\vulkan_cube\Release\qar-vulkan-source.exe'
if (-not (Test-Path -LiteralPath $exe)) { throw "$exe missing." }
Invoke-Native $exe @('--help')
Write-Host 'Vulkan cube build passed.' -ForegroundColor Green
