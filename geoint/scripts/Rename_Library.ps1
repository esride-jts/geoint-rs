# Dirty tweaking
if (Test-Path $args[0]) {
    Move-Item -Path $args[0] -Destination $args[1] -Force
}