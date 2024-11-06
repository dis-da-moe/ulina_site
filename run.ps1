param(
    [Parameter()]
    [Switch]$release = $false
)

$static_dir = "./server/static/"
$map_dir = $static_dir + "map/"
$html_name = "tools.html"
$public_dir = "./server/public/"

if ($release){
    trunk build ./tools/index.html --dist ($public_dir + "/tools") --filehash false --release
}
else{
    trunk build ./tools/index.html --dist ($public_dir + "/tools") --filehash false
}

Write-Output "running tailwind"
npx tailwindcss -i "./input.css" -o ($public_dir + "styles.css")


if ($release){
    cargo run --bin server --release
}
else{
    cargo run --bin server
}
