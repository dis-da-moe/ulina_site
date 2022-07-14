param(
    [Parameter()]
    [Switch]$release = $false
)

$build_from = "./tools/index.html"
$static_dir = "./server/static/"
$map_dir = $static_dir + "map/"
$html_name = "tools.html"
$public_dir = "./server/public/"


if ($release){
    trunk build $build_from --dist $map_dir --release
}
else{
    trunk build $build_from --dist $map_dir
}

Rename-Item -Path ($map_dir + "index.html") -NewName $html_name
Move-Item -Path ($map_dir + $html_name) -Destination $static_dir -Force

Remove-Item -Path ($public_dir + "*.*") | Where-Object { ! $_.PSIsContainer}

Get-Item -Path ($map_dir + "*") | Move-Item -Destination $public_dir -Force
Remove-Item -Path $map_dir

npx tailwindcss -i "./input.css" -o ($public_dir + "styles.css")

Write-Output "Moved Files"

if ($release){
    cargo run --bin server --release
}
else{
    cargo run --bin server
}
