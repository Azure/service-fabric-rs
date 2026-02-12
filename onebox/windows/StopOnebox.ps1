<#
.SYNOPSIS
    Stops the local Service Fabric onebox cluster.

.DESCRIPTION
    By default, only stops the FabricHostSvc service so the cluster can be
    restarted later with data intact. With -CleanData, fully removes node
    configuration, stops services, and deletes data/log folders.
    Auto-elevates to administrator via UAC if needed.

    Based on the Service Fabric SDK scripts:
      - "$env:ProgramFiles\Microsoft SDKs\Service Fabric\ClusterSetup\CleanCluster.ps1"
      - "$env:ProgramFiles\Microsoft SDKs\Service Fabric\Tools\Scripts\ClusterSetupUtilities.psm1"

.PARAMETER CleanData
    If present, fully removes the cluster including node configuration and
    data/log folders. Without this switch, only the service is stopped and
    the cluster can be restarted with 'Start-Service FabricHostSvc'.

.PARAMETER Auto
    If present, skips confirmation prompts before removing the cluster.

.EXAMPLE
    .\StopOnebox.ps1

.EXAMPLE
    .\StopOnebox.ps1 -CleanData

.EXAMPLE
    .\StopOnebox.ps1 -CleanData -Auto
#>
param
(
    [Parameter(Mandatory=$False)]
    [switch] $CleanData,

    [Parameter(Mandatory=$False)]
    [switch] $Auto
)

# Auto-elevate to admin if not already running as administrator
$identity = [Security.Principal.WindowsIdentity]::GetCurrent()
$principal = New-Object Security.Principal.WindowsPrincipal $identity
if (-not $principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)) {
    $args = @('-NoProfile', '-ExecutionPolicy', 'Bypass', '-File', "`"$PSCommandPath`"")
    if ($CleanData) { $args += '-CleanData' }
    if ($Auto) { $args += '-Auto' }
    Start-Process powershell.exe -ArgumentList $args -Verb RunAs -Wait
    exit $LASTEXITCODE
}

# Import ClusterSetupUtilities module
$sdkInstallPath = (Get-ItemProperty 'HKLM:\Software\Microsoft\Service Fabric SDK').FabricSDKScriptsPath
$modulePath = Join-Path -Path $sdkInstallPath -ChildPath "ClusterSetupUtilities.psm1"
Import-Module $modulePath

if (-not $CleanData) {
    # Just stop the service, preserving node configuration and data
    Write-Host "Stopping FabricHostSvc (keeping data and configuration)..."
    PerformServiceOperationWithWaitforStatus "FabricHostSvc" "Stop-Service" "Stopped" 10 5
    Write-Host ""
    Write-Host "Local Service Fabric Cluster stopped. Data preserved." -ForegroundColor Green
    Write-Host "Restart with: Start-Service FabricHostSvc" -ForegroundColor Green
    exit 0
}

if (!(IsLocalClusterSetup)) {
    Write-Host "No local Service Fabric Cluster is currently set up." -ForegroundColor Yellow
    exit 0
}

if (!$Auto.IsPresent) {
    Write-Warning "The local Service Fabric Cluster will be removed."
    $response = Read-Host -Prompt "Do you want to continue [Y/N]?"
    if ($response -ine "Y") { exit 0 }
}

CleanExistingCluster $False
CleanClusterDataLogFolders

Write-Host ""
Write-Host "Local Service Fabric Cluster removed successfully." -ForegroundColor Green
