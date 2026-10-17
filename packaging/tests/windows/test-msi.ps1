# Installs triib's MSI on this Windows and checks it as someone installing
# it would meet it: DLLs that only Windows itself provides, what the
# installer sets up, each program, the window, and what uninstalling
# leaves behind. It installs for the current user, replacing any triib
# installed, and removes it at the end.
#
#   packaging\tests\windows\test-msi.ps1 -Msi triib-1.2.0-x64.msi [-Previous triib-1.1.0-x64.msi] [-NoWindow]
#
# Previous, an older MSI, goes in first, so Msi upgrades it. NoWindow
# leaves out opening the window, for a Windows without a desktop, such as
# a container. Runs in Windows PowerShell 5.1 too, as that is all a clean
# Windows has.
param(
    [Parameter(Mandatory)] [string] $Msi,
    [string] $Previous = '',
    [switch] $NoWindow
)
$ErrorActionPreference = 'Stop'

# The DLLs Windows 10 and 11 themselves provide that the programs may
# use; the API sets (api-ms-win-*) are Windows' too. A DLL outside this
# list, such as the Visual C++ runtime's VCRUNTIME140.dll, is missing on
# a clean Windows, so it must be linked in or installed beside triib.
# Add one here only once sure every Windows triib supports has it.
$SystemDlls = @(
    'advapi32.dll', 'bcrypt.dll', 'bcryptprimitives.dll', 'combase.dll', 'comctl32.dll',
    'comdlg32.dll', 'crypt32.dll', 'd3d11.dll', 'd3d12.dll', 'd3dcompiler_47.dll',
    'dwmapi.dll', 'dwrite.dll', 'dxgi.dll', 'gdi32.dll', 'imm32.dll', 'iphlpapi.dll',
    'kernel32.dll', 'kernelbase.dll', 'mmdevapi.dll', 'ntdll.dll', 'ole32.dll',
    'oleaut32.dll', 'opengl32.dll', 'propsys.dll', 'secur32.dll', 'shcore.dll',
    'shell32.dll', 'shlwapi.dll', 'user32.dll', 'userenv.dll', 'uxtheme.dll',
    'version.dll', 'winmm.dll', 'ws2_32.dll'
)

function Section([string] $Text) { Write-Host "`n== $Text" }
function Fail([string] $Text) { throw "FAIL: $Text" }

# The DLL names a PE file imports, at load and delay-loaded.
function Get-Imports([string] $Path) {
    $bytes = [System.IO.File]::ReadAllBytes($Path)
    $pe = [BitConverter]::ToInt32($bytes, 0x3c)
    $sections = [BitConverter]::ToUInt16($bytes, $pe + 6)
    $optional = $pe + 24
    $optionalSize = [BitConverter]::ToUInt16($bytes, $pe + 20)
    $magic = [BitConverter]::ToUInt16($bytes, $optional)
    $directories = $optional + $(if ($magic -eq 0x20b) { 112 } else { 96 })
    $table = $optional + $optionalSize
    $offset = {
        param([uint32] $Rva)
        for ($index = 0; $index -lt $sections; $index++) {
            $header = $table + 40 * $index
            $address = [BitConverter]::ToUInt32($bytes, $header + 12)
            $size = [Math]::Max([BitConverter]::ToUInt32($bytes, $header + 8), [BitConverter]::ToUInt32($bytes, $header + 16))
            if ($Rva -ge $address -and $Rva -lt $address + $size) {
                return $Rva - $address + [BitConverter]::ToUInt32($bytes, $header + 20)
            }
        }
        throw "$Path has an address outside its sections"
    }
    $text = {
        param([uint32] $Rva)
        $start = & $offset $Rva
        $end = [Array]::IndexOf($bytes, [byte] 0, [int] $start)
        [System.Text.Encoding]::ASCII.GetString($bytes, $start, $end - $start)
    }
    $names = @()
    # Each import descriptor is 20 bytes, the name's address at 12; each
    # delay-load one is 32, the name's at 4.
    foreach ($kind in @(@{ Directory = 1; Size = 20; Name = 12 }, @{ Directory = 13; Size = 32; Name = 4 })) {
        $rva = [BitConverter]::ToUInt32($bytes, $directories + 8 * $kind.Directory)
        if ($rva -eq 0) { continue }
        $descriptor = & $offset $rva
        while ($true) {
            $name = [BitConverter]::ToUInt32($bytes, $descriptor + $kind.Name)
            if ($name -eq 0) { break }
            $names += (& $text $name).ToLowerInvariant()
            $descriptor += $kind.Size
        }
    }
    $names | Sort-Object -Unique
}

function Get-MsiProperty([string] $Path, [string] $Name) {
    $installer = New-Object -ComObject WindowsInstaller.Installer
    $database = $installer.GetType().InvokeMember('OpenDatabase', 'InvokeMethod', $null, $installer, @($Path, 0))
    $view = $database.GetType().InvokeMember('OpenView', 'InvokeMethod', $null, $database,
        @("SELECT Value FROM Property WHERE Property = '$Name'"))
    $view.GetType().InvokeMember('Execute', 'InvokeMethod', $null, $view, $null) | Out-Null
    $record = $view.GetType().InvokeMember('Fetch', 'InvokeMethod', $null, $view, $null)
    $value = $record.GetType().InvokeMember('StringData', 'GetProperty', $null, $record, 1)
    $view.GetType().InvokeMember('Close', 'InvokeMethod', $null, $view, $null) | Out-Null
    foreach ($object in @($record, $view, $database, $installer)) {
        [System.Runtime.InteropServices.Marshal]::ReleaseComObject($object) | Out-Null
    }
    $value
}

function Invoke-Msiexec([string[]] $Arguments, [string] $Log) {
    $run = Start-Process msiexec.exe -ArgumentList ($Arguments + @('/qn', '/l*v', "`"$Log`"")) -Wait -PassThru
    if ($run.ExitCode -notin @(0, 3010)) {
        # The verbose log's lines around the failing action.
        Select-String -Path $Log -Pattern 'Return value 3' -Context 30, 2 | Select-Object -First 1 | Out-Host
        Fail "msiexec $($Arguments -join ' ') ended with $($run.ExitCode)"
    }
}

# triib's entries in Apps, the installer's own uninstall entries.
function Get-Entries {
    $keys = @(
        'HKCU:\Software\Microsoft\Windows\CurrentVersion\Uninstall\*',
        'HKLM:\Software\Microsoft\Windows\CurrentVersion\Uninstall\*',
        'HKLM:\Software\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall\*'
    )
    @(Get-ItemProperty $keys -ErrorAction SilentlyContinue | Where-Object { $_.DisplayName -eq 'triib' })
}

function Get-UserPath {
    @([Environment]::GetEnvironmentVariable('Path', 'User') -split ';' | Where-Object { $_ } |
        ForEach-Object { $_.TrimEnd('\') })
}

# A GUI program's output, which PowerShell neither waits for nor shows.
function Get-Printed([string] $Program) {
    $printed = Join-Path $work 'printed.txt'
    $run = Start-Process $Program -ArgumentList '--version' -Wait -PassThru -NoNewWindow -RedirectStandardOutput $printed
    if ($run.ExitCode -ne 0) {
        Fail "$Program --version ended with $($run.ExitCode) ($('0x{0:X8}' -f $run.ExitCode))"
    }
    (Get-Content -Raw $printed).Trim()
}

$Msi = (Resolve-Path $Msi).Path
if ($Previous) { $Previous = (Resolve-Path $Previous).Path }
$work = Join-Path ([System.IO.Path]::GetTempPath()) "triib-msi-test-$PID"
New-Item -ItemType Directory -Force $work | Out-Null
$version = Get-MsiProperty $Msi 'ProductVersion'
$productCode = Get-MsiProperty $Msi 'ProductCode'
$folder = Join-Path $env:LOCALAPPDATA 'Programs\triib'
$os = Get-CimInstance Win32_OperatingSystem
$architecture = [System.Runtime.InteropServices.RuntimeInformation]::OSArchitecture
Section "$($os.Caption) $($os.Version), $($architecture): triib $version"

Section 'DLLs the programs need'
$image = Join-Path $work 'image'
Invoke-Msiexec @('/a', "`"$Msi`"", "TARGETDIR=`"$image`"") (Join-Path $work 'extract.log')
$binaries = @(Get-ChildItem $image -Recurse -Include *.exe, *.dll)
if ($binaries.Count -eq 0) { Fail 'the MSI holds no programs' }
$shipped = @($binaries | ForEach-Object { $_.Name.ToLowerInvariant() })
$problems = @()
foreach ($binary in $binaries) {
    $imports = @(Get-Imports $binary.FullName)
    Write-Host "$($binary.Name): $($imports -join ', ')"
    foreach ($import in $imports) {
        if ($import -like 'api-ms-win-*' -or $import -in $SystemDlls -or $import -in $shipped) { continue }
        $problems += "$($binary.Name) needs $import, which Windows does not have"
    }
    # On this Windows, as a container is a clean one.
    foreach ($import in $imports) {
        if ($import -like 'api-ms-win-*' -or $import -in $shipped) { continue }
        if (-not (Test-Path (Join-Path $env:SystemRoot "System32\$import"))) {
            Write-Host "  $import is not on this Windows"
        }
    }
}
if ($problems) { Fail ($problems -join '; ') }

if ($Previous) {
    Section "Install $(Split-Path -Leaf $Previous), to upgrade from"
    Invoke-Msiexec @('/i', "`"$Previous`"") (Join-Path $work 'previous.log')
    $previousCode = @(Get-Entries)[0].PSChildName
}

Section "Install $(Split-Path -Leaf $Msi)"
Invoke-Msiexec @('/i', "`"$Msi`"") (Join-Path $work 'install.log')

Section 'What it installed'
foreach ($program in @('triib.exe', 'triib-cli.exe')) {
    if (-not (Test-Path (Join-Path $folder $program))) { Fail "$folder\$program is missing" }
}
$entries = @(Get-Entries)
if ($entries.Count -ne 1) { Fail "Apps lists triib $($entries.Count) times" }
if ($entries[0].PSChildName -ne $productCode) { Fail "Apps lists $($entries[0].PSChildName), not $productCode" }
if ($entries[0].DisplayVersion -ne $version) { Fail "Apps lists version $($entries[0].DisplayVersion), not $version" }
if ($entries[0].Publisher -ne 'Scramble Tools') { Fail "Apps names $($entries[0].Publisher) as the publisher" }
if ($Previous -and $previousCode -eq $productCode) { Fail 'the previous version has the same product code' }
$onPath = @(Get-UserPath | Where-Object { $_ -eq $folder })
if ($onPath.Count -ne 1) { Fail "$folder is on the user's PATH $($onPath.Count) times" }
$shortcut = Join-Path ([Environment]::GetFolderPath('Programs')) 'triib.lnk'
if (-not (Test-Path $shortcut)) { Fail 'the Start menu has no triib' }
$target = (New-Object -ComObject WScript.Shell).CreateShortcut($shortcut).TargetPath
if ($target -ne (Join-Path $folder 'triib.exe')) { Fail "the Start menu's triib opens $target" }
Write-Host 'all there'

Section 'Each program'
# As a new terminal would find it, through the PATH the installer set.
$env:Path = [Environment]::GetEnvironmentVariable('Path', 'Machine') + ';' + [Environment]::GetEnvironmentVariable('Path', 'User')
$cli = (Get-Command triib-cli.exe -ErrorAction SilentlyContinue).Source
if ($cli -ne (Join-Path $folder 'triib-cli.exe')) { Fail "a new terminal finds triib-cli at '$cli'" }
foreach ($program in @('triib', 'triib-cli')) {
    $printed = Get-Printed (Join-Path $folder "$program.exe")
    Write-Host $printed
    if ($printed -ne "$program $version") { Fail "$program --version printed '$printed'" }
}

if (-not $NoWindow) {
    Section 'The window'
    $process = Start-Process (Join-Path $folder 'triib.exe') -PassThru
    $shown = $false
    for ($wait = 0; $wait -lt 60 -and -not $process.HasExited; $wait++) {
        Start-Sleep -Milliseconds 500
        $process.Refresh()
        if ($process.MainWindowHandle -ne 0) { $shown = $true; break }
    }
    if (-not $shown) {
        if ($process.HasExited) { Fail "triib quit before its window opened, with $('0x{0:X8}' -f $process.ExitCode)" }
        Stop-Process -Id $process.Id
        Fail "triib runs but its window did not open; a session without a desktop needs -NoWindow"
    }
    Start-Sleep -Seconds 5
    if ($process.HasExited) { Fail "triib quit after opening its window, with $('0x{0:X8}' -f $process.ExitCode)" }
    Write-Host "the window opened: $($process.MainWindowTitle)"
    Stop-Process -Id $process.Id
    $process.WaitForExit()
}

Section 'Uninstall'
Invoke-Msiexec @('/x', $productCode) (Join-Path $work 'uninstall.log')
$leftovers = @()
if (Test-Path $folder) { $leftovers += $folder }
if (Test-Path $shortcut) { $leftovers += $shortcut }
if (Get-UserPath | Where-Object { $_ -eq $folder }) { $leftovers += "$folder on the PATH" }
if (Test-Path 'HKCU:\Software\Scramble Tools\triib') { $leftovers += 'HKCU\Software\Scramble Tools\triib' }
if (@(Get-Entries).Count -ne 0) { $leftovers += 'the entry in Apps' }
if ($leftovers) { Fail "uninstalling left $($leftovers -join ', ')" }
Write-Host 'nothing left'

Section "Passed: $(Split-Path -Leaf $Msi)"
Remove-Item -Recurse -Force $work -ErrorAction SilentlyContinue
