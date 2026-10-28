<#
.SYNOPSIS
    The documentation website build (.github/workflows/docs-website.yml), runnable locally.

.DESCRIPTION
    In docs/website, from any working directory: `npm ci`, then `npm run build`, which generates
    the C API reference (Doxygen, a pinned copy downloaded by `npm ci`), the C# API reference
    (DocFX, pinned in dotnet-tools.json), the Rust API reference (rustdoc for
    x86_64-pc-windows-msvc, from this checkout and the release tags) and the diagrams, then builds
    the site into docs/website/build. Publishing it is CI's job.

    Needs Node.js 24 (CI's version; others print a warning), the .NET 8 SDK or newer, the latest
    stable Rust with the x86_64-pc-windows-msvc target, and the v<version> release tags fetched
    (`git fetch --tags`).

.EXAMPLE
    ./ci/docs-website.ps1
#>
param()

. "$PSScriptRoot/common.ps1"

$websiteDirectory = Join-Path $QarosRoot 'docs/website'
$rustTarget = 'x86_64-pc-windows-msvc'

Assert-Tool node 'install Node.js 24 from https://nodejs.org'
Assert-Tool npm 'install Node.js 24 from https://nodejs.org (npm comes with it)'
$nodeVersion = (& node --version).Trim()
if ($nodeVersion -notmatch '^v24\.') { Write-Warning "Node.js ${nodeVersion}: CI builds with Node.js 24." }
Assert-Tool dotnet 'install the .NET 8 SDK (or newer) from https://dot.net'
if (-not ((& dotnet --list-sdks) -match '^(8|9|[1-9][0-9])\.')) {
    throw "Missing the .NET SDK 8 or newer (DocFX generates the C# API reference): install it from https://dot.net."
}
Assert-Tool cargo 'install the latest stable Rust from https://rustup.rs'
Assert-Tool rustup 'install the latest stable Rust from https://rustup.rs'
if (-not ((& rustup target list --installed) -contains $rustTarget)) {
    throw "Missing the Rust target $rustTarget (rustdoc documents the binding for it): run 'rustup target add $rustTarget'."
}
Assert-Tool git 'install Git from https://git-scm.com'
if (-not (& git -C $QarosRoot tag --list 'v*')) {
    throw "No v<version> tags in $QarosRoot (the Rust API reference of each release is built from its tag): run 'git fetch --tags'."
}

Invoke-Native npm @('ci') $websiteDirectory
Invoke-Native npm @('run', 'build') $websiteDirectory
Write-Host "Docs website built: $(Join-Path $websiteDirectory 'build')" -ForegroundColor Green
