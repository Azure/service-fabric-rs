param(
    [Parameter(Mandatory = $true)]
    [ValidateSet("Add", "Remove")]
    [string] $Action
)

$ErrorActionPreference = "Stop"

$path = "build\sf_apps\dummy-self-reconfig"
$imageStorePath = "DummySelfReconfigApplicationV1"

Connect-ServiceFabricCluster

if ($Action -eq "Add") {
    Test-ServiceFabricApplicationPackage -ApplicationPackagePath $path
    Copy-ServiceFabricApplicationPackage `
        -ApplicationPackagePath $path `
        -ApplicationPackagePathInImageStore $imageStorePath `
        -TimeoutSec 1800
    Register-ServiceFabricApplicationType -ApplicationPathInImageStore $imageStorePath
    New-ServiceFabricApplication `
        -ApplicationName fabric:/DummySelfReconfigApp `
        -ApplicationTypeName DummySelfReconfigAppType `
        -ApplicationTypeVersion 1.0
} else {
    Remove-ServiceFabricApplication fabric:/DummySelfReconfigApp -Force
    Unregister-ServiceFabricApplicationType DummySelfReconfigAppType 1.0 -Force
    Remove-ServiceFabricApplicationPackage `
        -ApplicationPackagePathInImageStore $imageStorePath `
        -Force
}
