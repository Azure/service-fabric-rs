param(
    [ValidateSet("Add","Remove","Connect", "Resolve", "Echo")]
    [String]
    $Action,
    [ValidateSet("Manual", "Test")]
    [String]
    $Mode = "Manual"
)
$ErrorActionPreference = "Stop";

$path = "build\echoapp_root_stateful2"
Write-Host "path: $path"
if($Action -eq "Connect"){
    Connect-ServiceFabricCluster
}elseif ($Action -eq "Add") {
    Connect-ServiceFabricCluster
    Test-ServiceFabricApplicationPackage -ApplicationPackagePath $path

    Copy-ServiceFabricApplicationPackage -ApplicationPackagePath $path -ApplicationPackagePathInImageStore MyApplicationV1 -TimeoutSec 1800
    
    Register-ServiceFabricApplicationType -ApplicationPathInImageStore MyApplicationV1
    New-ServiceFabricApplication fabric:/StatefulEchoApp StatefulEchoApp 0.0.1
}elseif($Action -eq "Remove"){
    Connect-ServiceFabricCluster
    Remove-ServiceFabricApplication fabric:/StatefulEchoApp -Force
    Unregister-ServiceFabricApplicationType StatefulEchoApp 0.0.1 -Force
    Remove-ServiceFabricApplicationPackage -ApplicationPackagePathInImageStore MyApplicationV1 -Force
}elseif($Action -eq "Resolve"){
    Connect-ServiceFabricCluster
    Resolve-ServiceFabricService -ServiceName fabric:/StatefulEchoApp/StatefulEchoAppService -PartitionKindUniformInt64 -ForceRefresh
}elseif($Action -eq "Echo"){
    Connect-ServiceFabricCluster
    $resolve = Resolve-ServiceFabricService -ServiceName fabric:/StatefulEchoApp/StatefulEchoAppService -PartitionKindUniformInt64 -ForceRefresh
    Write-Host $resolve
    $addr = $resolve.Endpoints.Address
    $pair = $addr.Split(":")
    $hostname = $pair[0]
    $port = $pair[1] 

    $tcpConnection = New-Object System.Net.Sockets.TcpClient("$hostname", "$port")
    $tcpStream = $tcpConnection.GetStream()
    $reader = New-Object System.IO.StreamReader($tcpStream)
    $writer = New-Object System.IO.StreamWriter($tcpStream)
    $writer.AutoFlush = $true

    while ($tcpConnection.Connected)
    {
        if ($tcpConnection.Connected)
        {
            if($Mode -eq "Test"){
                $writer.WriteLine("hello") | Out-Null
            }else{
                Write-Host -NoNewline "prompt> "
                $command = Read-Host

                if ($command -eq "escape")
                {
                    break
                }

                $writer.WriteLine($command) | Out-Null
            }
        }
        while ($tcpStream.DataAvailable -or $reader.Peek() -ne -1 ) {
            if($Mode -eq "Test"){
                $reply = $reader.ReadLine()
                if($reply -ne "hello")
                {
                    Write-Error "reply $reply is not hello"
                    Exit 1
                }
                Write-Host "Echo test success"
                break
            }else{
                Write-Host $reader.ReadLine()
            }
        }

        if($Mode -eq "Test"){
            break
        }
        start-sleep -Milliseconds 500
    }

    $reader.Close()
    $writer.Close()
    $tcpConnection.Close()
}


#   Get-ServiceFabricApplicationType
#   Get-ServiceFabricApplication
