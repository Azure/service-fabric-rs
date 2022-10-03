param(
    [ValidateSet("Add","Remove","Connect", "Resolve", "Echo")]
    [String]
    $Action,
    [ValidateSet("Manual", "Test")]
    [String]
    $Mode = "Manual"
)
$ErrorActionPreference = "Stop";

#pass through script to cpp repo
.\build\_deps\service_fabric_cpp-src\scripts\echomain_ctl.ps1 -Action $Action -Mode $Mode