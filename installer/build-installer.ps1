# Script PowerShell pour compiler l'installeur Z-Cleaner avec Inno Setup
# Utilisation : .\build-installer.ps1

param(
    [string]$InnoSetupPath = "C:\Program Files (x86)\Inno Setup 6",
    [string]$IssFile = "installer\z-cleaner-installer.iss",
    [string]$OutputDir = "installer\output"
)

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "Z-Cleaner Installer Builder" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# Vérifier si Inno Setup est installé
$InnoSetupExe = Join-Path $InnoSetupPath "ISCC.exe"

if (-not (Test-Path $InnoSetupExe)) {
    Write-Host "❌ Erreur: Inno Setup n'est pas trouvé à $InnoSetupPath" -ForegroundColor Red
    Write-Host "Veuillez installer Inno Setup 6 depuis https://jrsoftware.org/isdl.php" -ForegroundColor Yellow
    exit 1
}

Write-Host "✓ Inno Setup trouvé: $InnoSetupExe" -ForegroundColor Green
Write-Host ""

# Vérifier si le fichier .iss existe
if (-not (Test-Path $IssFile)) {
    Write-Host "❌ Erreur: Fichier $IssFile non trouvé" -ForegroundColor Red
    exit 1
}

Write-Host "✓ Fichier de configuration trouvé: $IssFile" -ForegroundColor Green
Write-Host ""

# Créer le dossier de sortie s'il n'existe pas
if (-not (Test-Path $OutputDir)) {
    New-Item -ItemType Directory -Path $OutputDir | Out-Null
    Write-Host "✓ Dossier de sortie créé: $OutputDir" -ForegroundColor Green
}

Write-Host ""
Write-Host "Compilation de l'installeur en cours..." -ForegroundColor Yellow
Write-Host ""

# Compiler avec Inno Setup
& $InnoSetupExe $IssFile

# Vérifier si la compilation a réussi
if ($LASTEXITCODE -eq 0) {
    Write-Host ""
    Write-Host "========================================" -ForegroundColor Green
    Write-Host "✓ Compilation réussie !" -ForegroundColor Green
    Write-Host "========================================" -ForegroundColor Green
    Write-Host ""
    Write-Host "L'installeur a été créé dans: $OutputDir" -ForegroundColor Green
    
    # Afficher les fichiers créés
    $SetupFiles = Get-ChildItem -Path $OutputDir -Filter "*.exe" | Sort-Object -Property LastWriteTime -Descending
    if ($SetupFiles) {
        Write-Host ""
        Write-Host "Fichiers créés:" -ForegroundColor Cyan
        foreach ($file in $SetupFiles) {
            $size = [math]::Round($file.Length / 1MB, 2)
            Write-Host "  - $($file.Name) ($size MB)" -ForegroundColor Green
        }
    }
} else {
    Write-Host ""
    Write-Host "========================================" -ForegroundColor Red
    Write-Host "❌ Erreur lors de la compilation" -ForegroundColor Red
    Write-Host "========================================" -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "Terminé !" -ForegroundColor Green
