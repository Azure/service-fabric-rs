$ErrorActionPreference = "Stop";
#Set-PSDebug -Trace 1

$sfdir = "C:\Program Files\Microsoft Service Fabric"

If (!(test-path $sfdir))
{
    Write-Error "SF not found"
    Exit 1
}

Write-Output "SF found"
Get-ItemPropertyValue 'HKLM:\SOFTWARE\Microsoft\Service Fabric\' -Name FabricVersion

$sfSDKDir = "C:\Program Files\Microsoft SDKs\Service Fabric"
If (!(test-path $sfSDKDir))
{
    Write-Error "SF SDK not found"
    Exit 1
}

Write-Output "SF SDK found"

Import-Module -Name ServiceFabric

# The cluster does not seem to be running by default
# try connect to cluster
# Connect-ServiceFabricCluster