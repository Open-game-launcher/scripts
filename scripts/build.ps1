$targets = @("x86_64-unknown-linux-gnu", "x86_64-pc-windows-gnu")

foreach ($target in $targets) {
    Write-Host "Compiling for $target with cross..."
    cross build --target $target --release
}