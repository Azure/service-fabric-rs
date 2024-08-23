$ErrorActionPreference = "Stop";

# Script root is the current script file location

& "$PSScriptRoot\..\scripts\echomain_ctl.ps1" -Action Add

start-sleep -seconds 20

& "$PSScriptRoot\..\scripts\echomain_ctl.ps1" -Action Resolve

& "$PSScriptRoot\..\scripts\echomain_ctl.ps1" -Action Echo -Mode Test

& "$PSScriptRoot\..\scripts\echomain_ctl.ps1" -Action Remove