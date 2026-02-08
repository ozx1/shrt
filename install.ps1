$ErrorActionPreference = "Stop"

# Force TLS 1.2
[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12

Write-Host "Installing shrt for Windows..." -ForegroundColor Green

try {
    # Get latest release info from GitHub API
    Write-Host "Fetching latest release..." -ForegroundColor Cyan
    
    $webClient = New-Object System.Net.WebClient
    $webClient.Headers.Add("User-Agent", "PowerShell")
    
    $apiResponse = $webClient.DownloadString("https://api.github.com/repos/ozx1/shrt/releases/latest")
    $release = $apiResponse | ConvertFrom-Json
    $version = $release.tag_name
    
    # Build download URL with actual version
    $downloadUrl = "https://github.com/ozx1/shrt/releases/download/$version/shrt-windows-x86_64.exe"
    $tempFile = Join-Path $env:TEMP "shrt.exe"
    
    Write-Host "Downloading version $version from GitHub..." -ForegroundColor Cyan
    
    # Download using WebClient (more reliable than Invoke-WebRequest)
    $webClient.DownloadFile($downloadUrl, $tempFile)
    
    Write-Host "Download complete!" -ForegroundColor Green
    
    # Create install directory
    $installDir = Join-Path $env:LOCALAPPDATA "Programs\shrt"
    if (-not (Test-Path $installDir)) {
        New-Item -ItemType Directory -Path $installDir -Force | Out-Null
    }
    
    # Move file to install directory
    $finalPath = Join-Path $installDir "shrt.exe"
    if (Test-Path $finalPath) {
        Remove-Item $finalPath -Force
    }
    Move-Item $tempFile $finalPath -Force
    
    Write-Host "Installed to: $finalPath" -ForegroundColor Green
    
    # Add to PATH
    $userPath = [Environment]::GetEnvironmentVariable("Path", "User")
    if ($userPath -notlike "*$installDir*") {
        $newPath = $userPath + ";" + $installDir
        [Environment]::SetEnvironmentVariable("Path", $newPath, "User")
        $env:Path = [Environment]::GetEnvironmentVariable("Path", "Machine") + ";" + $newPath
        Write-Host "Added to PATH" -ForegroundColor Green
        Write-Host "Note: You may need to restart your terminal for PATH changes to take effect" -ForegroundColor Yellow
    } else {
        Write-Host "Already in PATH" -ForegroundColor Green
    }
    
    Write-Host ""
    Write-Host "Installation complete!" -ForegroundColor Green
    Write-Host "Run 'shrt help' to get started (restart your terminal first if needed)" -ForegroundColor Cyan
    
} catch {
    Write-Host "Installation failed: $_" -ForegroundColor Red
    Write-Host ""
    Write-Host "Manual installation:" -ForegroundColor Yellow
    Write-Host "1. Download: https://github.com/ozx1/shrt/releases/download/v0.1.0/shrt-windows-x86_64.exe" -ForegroundColor Yellow
    Write-Host "2. Rename to shrt.exe" -ForegroundColor Yellow
    Write-Host "3. Move to a folder in your PATH" -ForegroundColor Yellow
    exit 1
}