<#
.SYNOPSIS
    Sets up a local Service Fabric onebox cluster.

.DESCRIPTION
    Sets up a local Service Fabric cluster using the ClusterManifestTemplate.json
    in the same directory. Auto-elevates to administrator via UAC if needed.

    Based on the Service Fabric SDK scripts:
      - "$env:ProgramFiles\Microsoft SDKs\Service Fabric\ClusterSetup\DevClusterSetup.ps1"
      - "$env:ProgramFiles\Microsoft SDKs\Service Fabric\Tools\Scripts\ClusterSetupUtilities.psm1"

.PARAMETER PathToClusterDataRoot
    Path to the cluster data root directory. If not specified, defaults to the
    value in ClusterManifestTemplate.json (e.g. C:\SfDevCluster\Data).

.PARAMETER PathToClusterLogRoot
    Path to the cluster log root directory. If not specified, defaults to the
    value in ClusterManifestTemplate.json (e.g. C:\SfDevCluster\Log).

.PARAMETER Clean
    If present, removes any existing cluster before setting up a new one.
    Without this switch, the script will error if a cluster already exists.

.PARAMETER Auto
    If present, skips confirmation prompts (e.g. before removing an existing cluster).

.EXAMPLE
    .\StartOnebox.ps1

.EXAMPLE
    .\StartOnebox.ps1 -Clean -Auto

.EXAMPLE
    .\StartOnebox.ps1 -PathToClusterDataRoot "D:\SfCluster\Data" -PathToClusterLogRoot "D:\SfCluster\Log" -Auto
#>
param
(
    [Parameter(Mandatory=$False)]
    [string] $PathToClusterDataRoot = "",

    [Parameter(Mandatory=$False)]
    [string] $PathToClusterLogRoot = "",

    [Parameter(Mandatory=$False)]
    [switch] $Clean,

    [Parameter(Mandatory=$False)]
    [switch] $Auto
)

# Auto-elevate to admin if not already running as administrator
$identity = [Security.Principal.WindowsIdentity]::GetCurrent()
$principal = New-Object Security.Principal.WindowsPrincipal $identity
if (-not $principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)) {
    $args = @('-NoProfile', '-ExecutionPolicy', 'Bypass', '-File', "`"$PSCommandPath`"")
    if ($PathToClusterDataRoot) { $args += "-PathToClusterDataRoot `"$PathToClusterDataRoot`"" }
    if ($PathToClusterLogRoot) { $args += "-PathToClusterLogRoot `"$PathToClusterLogRoot`"" }
    if ($Clean) { $args += '-Clean' }
    if ($Auto) { $args += '-Auto' }
    Start-Process powershell.exe -ArgumentList $args -Verb RunAs -Wait
    exit $LASTEXITCODE
}

# Import ClusterSetupUtilities module
$sdkInstallPath = (Get-ItemProperty 'HKLM:\Software\Microsoft\Service Fabric SDK').FabricSDKScriptsPath
$modulePath = Join-Path -Path $sdkInstallPath -ChildPath "ClusterSetupUtilities.psm1"
Import-Module $modulePath

# Handle existing cluster
if (IsLocalClusterSetup) {
    if ($Clean) {
        if (!$Auto.IsPresent) {
            Write-Warning "A local Service Fabric Cluster already exists and will be removed."
            $response = Read-Host -Prompt "Do you want to continue [Y/N]?"
            if ($response -ine "Y") { exit 0 }
        }
        CleanExistingCluster
    } else {
        # Cluster already configured, just start the service
        Write-Host "Local Service Fabric Cluster is already set up. Starting service..."
        StartLocalCluster
        $connParams = GetConnectionParameters
        TryConnectToCluster -connParams $connParams -waitTime 240
        CheckNamingServiceReady -connParams $connParams -waitTime 120
        Write-Host ""
        Write-Host "Local Service Fabric Cluster started successfully." -ForegroundColor Green
        Write-Host "Connect via PowerShell: Connect-ServiceFabricCluster" -ForegroundColor Green
        Write-Host "Explorer: http://localhost:19080/Explorer" -ForegroundColor Green
        exit 0
    }
}

# Use the JSON template from the same directory as this script
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$jsonFileTemplate = Join-Path $ScriptDir "ClusterManifestTemplate.json"

if (!(Test-Path $jsonFileTemplate)) {
    Write-Error "JSON template not found at: $jsonFileTemplate"
    exit 1
}

# Set up data and log roots
$clusterRoots = SetupDataAndLogRoot -clusterDataRoot $PathToClusterDataRoot `
                                     -clusterLogRoot $PathToClusterLogRoot `
                                     -jsonFileTemplate $jsonFileTemplate `
                                     -isAuto $Auto.IsPresent

if ($clusterRoots[0] -eq $False) { exit 0 }

$clusterDataRoot = $clusterRoots[0]
$clusterLogRoot  = $clusterRoots[1]

# Instantiate the JSON template with data/log root paths
$jsonTemplate = InstantiateJsonFromTemplate -jsonFileTemplate $jsonFileTemplate `
                                            -clusterDataRoot $clusterDataRoot `
                                            -clusterLogRoot $clusterLogRoot `
                                            -configureFirewall $False

# Build cluster manifest XML from the JSON
$manifestFileTemplate = ConstructManifestFileTemplate -jsonTemplate $jsonTemplate

# Prepare image store and machine name
$imageStoreConnectionString = SetupImageStore -clusterDataRoot $clusterDataRoot -useImageStoreService $False
$machineName = GetMachineName -useMachineName $True

# Prepare the final cluster manifest (replaces placeholders)
$manifestFile = PrepareClusterManifest -manifestFileTemplate $manifestFileTemplate `
                                       -imageStoreConnectionString $imageStoreConnectionString `
                                       -machineName $machineName

# Stop FabricHostSvc if running
PerformServiceOperationWithWaitforStatus "FabricHostSvc" "Stop-Service" "Stopped" 10 5

# Deploy node configuration
New-ServiceFabricNodeConfiguration -ClusterManifest "$manifestFile" `
                                   -FabricDataRoot "$clusterDataRoot" `
                                   -FabricLogRoot "$clusterLogRoot" `
                                   -RunFabricHostServiceAsManual
if (!$?) {
    Write-Error "Could not create node configuration for '$manifestFile'"
    exit 1
}

# Record cluster info in registry
Set-ItemProperty 'HKLM:\Software\Microsoft\Service Fabric SDK' -Name LocalClusterNodeCount -Value 5
Set-ItemProperty 'HKLM:\Software\Microsoft\Service Fabric SDK' -Name IsMeshCluster -Value "false"

# Save connection parameters
SaveConnectionParameters -dataRoot $clusterDataRoot

# Start the cluster
StartLocalCluster

# Wait for the cluster to become ready
$connParams = GetConnectionParameters
TryConnectToCluster -connParams $connParams -waitTime 240
CheckNamingServiceReady -connParams $connParams -waitTime 120

Write-Host ""
Write-Host "Local Service Fabric Cluster created successfully." -ForegroundColor Green
Write-Host "Connect via PowerShell: Connect-ServiceFabricCluster" -ForegroundColor Green
Write-Host "Explorer: http://localhost:19080/Explorer" -ForegroundColor Green
