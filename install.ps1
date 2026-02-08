$ErrorActionPreference = "Stop"

# Force TLS 1.2 for older PowerShell versions
[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12

Write-Host "Installing shrt for Windows..." -ForegroundColor Green

# Get latest release
$release = Invoke-RestMethod -Uri "https://api.github.com/repos/ozx1/shrt/releases/latest"
$version = $release.tag_name
$asset = $release.assets | Where-Object { $_.name -eq "shrt-windows-x86_64.exe" }
$downloadUrl = $asset.browser_download_url

Write-Host "Downloading version $version..." -ForegroundColor Cyan
$tempFile = "$env:TEMP\shrt.exe"

# Use WebClient for direct download (avoids HTML response issues)
$webClient = New-Object System.Net.WebClient
$webClient.DownloadFile($downloadUrl, $tempFile)

# Install to user's local bin
$installDir = "$env:LOCALAPPDATA\Programs\shrt"
New-Item -ItemType Directory -Force -Path $installDir | Out-Null
Move-Item -Force $tempFile "$installDir\shrt.exe"

# Add to PATH if not already there
$userPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($userPath -notlike "*$installDir*") {
    [Environment]::SetEnvironmentVariable("Path", "$userPath;$installDir", "User")
    Write-Host "Added to PATH. Please restart your terminal." -ForegroundColor Yellow
}

Write-Host "✓ shrt installed successfully!" -ForegroundColor Green
Write-Host "Run 'shrt help' to get started" -ForegroundColor Cyan