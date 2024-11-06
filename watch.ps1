$trunk = Start-Job -ScriptBlock{ Set-Location $using:PWD; trunk watch ./tools/index.html --dist ./server/public/tools --filehash false} -name "trunk"
$tailwind = Start-Job -ScriptBlock{ Set-Location $using:PWD; ./tailwind.ps1} -name "tailwind"
$server = Start-Job -ScriptBlock{Set-Location $using:PWD; cargo run --bin server} -name "server"

$jobs = @($trunk, $tailwind, $server)

try{
    While(1){
        foreach($job in $jobs){
            Write-Host $(Receive-Job -Job $job) -NoNewline
        }
        Start-Sleep 1
    }
}
finally{
    foreach($job in $jobs){
        Stop-Job $job
        Remove-Job $job
    }
}


