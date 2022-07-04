param(
    [Parameter()]
    [Switch]$release = $false
)

$build_from = "./ulina_map/index.html"
$static_dir = "./ulina_server/static/"
$map_dir = $static_dir + "map/"
$html_name = "map.html"
$public_dir = "./ulina_server/public/"


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
    cargo run --bin ulina_server --release
}
else{
    cargo run --bin ulina_server
}
