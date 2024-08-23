$ErrorActionPreference = "Stop";

Import-Module -Name ServiceFabric

[array]$a = 1..10

$Failed = $true

foreach ($i in $a)
{
    Try{
        Connect-ServiceFabricCluster
    } catch { 
        Write-Output "Failed to connect attemp [$i]"
        Start-Sleep -Seconds 5
        continue
    }
    # success
    $Failed = $false
    break
}

if($Failed){
    Write-Output "Connect failed."
    Exit 1
}
Write-Output "Connect success"

