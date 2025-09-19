function __name__ {
    $previousCmd = (Get-History -Count 1).CommandLine
    try {
        $env:FIXIT_FNS = (Get-Command).Name
        # trimming is required to make Add-History work
        $fixedCmd = (fixit fix --powershell "$previousCmd" | Out-String).Trim()
    } finally {
        Remove-Item env:FIXIT_FNS
    }
    if ( $fixedCmd -ne '' ) {
        $startTime = Get-Date
        Invoke-Expression $fixedCmd
        $status = if ($?) { "Completed" } else { "Failed" }
        $endTime = Get-Date
        $history = [pscustomobject]@{
            CommandLine = $fixedCmd
            ExecutionStatus = $status
            StartExecutionTime = $startTime
            EndExecutionTime = $endTime
        }
        Add-History -InputObject @($history)
    }
}
