<#
.SYNOPSIS
    The checks of the Rust binding workflow (.github/workflows/rust-binding.yml), runnable locally.

.DESCRIPTION
    Runs in qar-streaming-rust/, from any working directory:
      Format     cargo fmt --all -- --check
      Clippy     cargo clippy --workspace --all-targets with the vulkan and e2e features, -D warnings
      Test       cargo test --workspace (against qaros-stub, no QAROS needed)
      Tutorials  cargo xtask tutorials --check
      Guide      cargo xtask guide --check
    Without -Step it runs these five, in this order, as CI does.

      E2E        cargo test -p qar-streaming-rust-ext --features e2e: the integration tests against
                 qar-streaming-c-ext. Not part of CI (hosted runners have no SDK build). Needs
                 QAROS_NATIVE_BIN: the folder holding qar-streaming-c.dll, qar-streaming-c-ext.dll
                 and every DLL they load (the bin folder of a Debug or RelWithDebInfo SDK build).

.EXAMPLE
    ./ci/rust-binding.ps1
.EXAMPLE
    ./ci/rust-binding.ps1 -Step Clippy,Test
.EXAMPLE
    $env:QAROS_NATIVE_BIN = 'C:\sdk\bin'; ./ci/rust-binding.ps1 -Step E2E
#>
param(
    [ValidateSet('Format', 'Clippy', 'Test', 'Tutorials', 'Guide', 'E2E')]
    [string[]]$Step = @('Format', 'Clippy', 'Test', 'Tutorials', 'Guide')
)

. "$PSScriptRoot/common.ps1"

$rustDirectory = Join-Path $QarosRoot 'qar-streaming-rust'
$rustupHint = 'install the latest stable Rust from https://rustup.rs'

Assert-Tool cargo $rustupHint
if ($Step -contains 'Format' -and -not (Test-NativeCommand cargo @('fmt', '--version'))) {
    throw "Missing tool 'rustfmt': run 'rustup component add rustfmt'."
}
if ($Step -contains 'Clippy' -and -not (Test-NativeCommand cargo @('clippy', '--version'))) {
    throw "Missing tool 'clippy': run 'rustup component add clippy'."
}
if ($Step -contains 'E2E') {
    if (-not $env:QAROS_NATIVE_BIN) {
        throw ('QAROS_NATIVE_BIN is not set. Set it to the folder holding qar-streaming-c.dll, ' +
            'qar-streaming-c-ext.dll and their dependencies (the bin folder of a Debug or RelWithDebInfo SDK build). ' +
            'Inside the Quaternar repository, run .\utils\Invoke-QarosBindingTests.ps1 from its root instead.')
    }
    foreach ($dll in 'qar-streaming-c.dll', 'qar-streaming-c-ext.dll') {
        if (-not (Test-Path -LiteralPath (Join-Path $env:QAROS_NATIVE_BIN $dll))) {
            throw "QAROS_NATIVE_BIN is '$env:QAROS_NATIVE_BIN', which holds no $dll."
        }
    }
}

foreach ($name in $Step) {
    Write-Host "== $name" -ForegroundColor Green
    switch ($name) {
        'Format' { Invoke-Native cargo @('fmt', '--all', '--', '--check') $rustDirectory }
        'Clippy' {
            Invoke-Native cargo @('clippy', '--workspace', '--all-targets',
                '--features', 'qaros/vulkan,qaros-examples/vulkan,qar-streaming-rust-ext/e2e',
                '--', '-D', 'warnings') $rustDirectory
        }
        'Test' { Invoke-Native cargo @('test', '--workspace') $rustDirectory }
        'Tutorials' { Invoke-Native cargo @('xtask', 'tutorials', '--check') $rustDirectory }
        'Guide' { Invoke-Native cargo @('xtask', 'guide', '--check') $rustDirectory }
        'E2E' {
            Invoke-Native cargo @('test', '-p', 'qar-streaming-rust-ext', '--features', 'e2e',
                '--', '--nocapture') $rustDirectory
        }
    }
}
Write-Host "Rust binding: $($Step -join ', ') passed." -ForegroundColor Green
