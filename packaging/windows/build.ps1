# Packages a Windows build of triib: the MSI installer and a portable zip.
#
#   packaging\windows\build.ps1 -Version 1.2.0 -Source dist -Out out [-Arch arm64] [-Icon triib.ico]
#
# Source holds triib.exe and triib-cli.exe. Needs WiX 5 and its UI and
# Util extensions:
#   dotnet tool install --global wix --version 5.0.2
#   wix extension add -g WixToolset.UI.wixext/5.0.2 WixToolset.Util.wixext/5.0.2
param(
    [Parameter(Mandatory)] [string] $Version,
    [Parameter(Mandatory)] [string] $Source,
    [Parameter(Mandatory)] [string] $Out,
    [ValidateSet('x64', 'arm64')] [string] $Arch = 'x64',
    [string] $Icon = ''
)
$ErrorActionPreference = 'Stop'
$here = Split-Path -Parent $MyInvocation.MyCommand.Path
$root = Resolve-Path (Join-Path $here '..\..')
New-Item -ItemType Directory -Force $Out | Out-Null

$msi = Join-Path $Out "triib-$Version-$Arch.msi"
$defines = @('-d', "Version=$Version", '-d', "Source=$((Resolve-Path $Source).Path)")
if ($Icon) { $defines += @('-d', "Icon=$((Resolve-Path $Icon).Path)") }
wix build (Join-Path $here 'triib.wxs') -arch $Arch -ext WixToolset.UI.wixext -ext WixToolset.Util.wixext @defines -o $msi
if ($LASTEXITCODE -ne 0) { throw "wix build failed" }
# WiX's debug symbols are no use to people installing triib.
Remove-Item (Join-Path $Out '*.wixpdb') -ErrorAction SilentlyContinue

$zip = Join-Path $Out "triib-$Version-$Arch-windows.zip"
$stage = Join-Path ([System.IO.Path]::GetTempPath()) "triib-$Version-$Arch"
Remove-Item -Recurse -Force $stage -ErrorAction SilentlyContinue
New-Item -ItemType Directory $stage | Out-Null
Copy-Item (Join-Path $Source 'triib.exe'), (Join-Path $Source 'triib-cli.exe') $stage
Copy-Item (Join-Path $root 'LICENSE-MIT'), (Join-Path $root 'LICENSE-APACHE'), (Join-Path $root 'README.md') $stage
New-Item -ItemType Directory (Join-Path $stage 'fonts') | Out-Null
Copy-Item (Join-Path $root 'packaging\licenses\*') (Join-Path $stage 'fonts')
Compress-Archive -Path (Join-Path $stage '*') -DestinationPath $zip -Force

Get-ChildItem $Out
