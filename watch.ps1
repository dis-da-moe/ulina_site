$trunk = Start-Job -ScriptBlock{ Set-Location $using:PWD; trunk watch ./tools/index.html --dist ./server/public/tools --filehash false} -name "trunk"
$tailwind = Start-Job -ScriptBlock{ Set-Location $using:PWD; npx tailwindcss -i "./input.css" -o "./server/public/styles.css" -w} -name "tailwind"

$jobs = @($trunk, $tailwind)

try{
    While(2){
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


