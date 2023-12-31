param(
    [ValidateSet("Add","Remove","Connect", "Resolve", "Echo")]
    [String]
    $Action,
    [ValidateSet("Manual", "Test")]
    [String]
    $Mode = "Manual"
)
$ErrorActionPreference = "Stop";

$path = "build\echo2_root"

if($Action -eq "Connect"){
    Connect-ServiceFabricCluster
}elseif ($Action -eq "Add") {
    Connect-ServiceFabricCluster
    Test-ServiceFabricApplicationPackage -ApplicationPackagePath $path

    Copy-ServiceFabricApplicationPackage -ApplicationPackagePath $path -ApplicationPackagePathInImageStore MyApplicationV1 -TimeoutSec 1800
    
    Register-ServiceFabricApplicationType -ApplicationPathInImageStore MyApplicationV1
    New-ServiceFabricApplication fabric:/EchoApp2 EchoApp2 0.0.1
}elseif($Action -eq "Remove"){
    Connect-ServiceFabricCluster
    Remove-ServiceFabricApplication fabric:/EchoApp2 -Force
    Unregister-ServiceFabricApplicationType EchoApp2 0.0.1 -Force
    Remove-ServiceFabricApplicationPackage -ApplicationPackagePathInImageStore MyApplicationV1 -Force
}elseif($Action -eq "Resolve"){
    Connect-ServiceFabricCluster
    Resolve-ServiceFabricService -ServiceName fabric:/EchoApp2/EchoAppService -PartitionKindSingleton -ForceRefresh
}elseif($Action -eq "Echo"){
 
}


#   Get-ServiceFabricApplicationType
#   Get-ServiceFabricApplication
