# Guide d'Installation - Z-Cleaner Installer avec Inno Setup

## 📋 Vue d'ensemble

Ce guide vous montre comment créer un installeur professionnel pour Z-Cleaner en utilisant Inno Setup.

## 🔧 Prérequis

### Windows (Recommandé)
- **Inno Setup 6** (gratuit) : https://jrsoftware.org/isdl.php
- **Tauri** compilé (voir ci-dessous)
- **PowerShell** (inclus dans Windows)

### macOS/Linux
- **Wine** (pour exécuter Inno Setup)
- **Tauri** compilé

## 📁 Structure des Fichiers

```
z-cleaner-2/
├── installer/
│   ├── z-cleaner-installer.iss          # Configuration Inno Setup
│   ├── EULA_FR.txt                      # Contrat de licence
│   ├── WELCOME_FR.txt                   # Message de bienvenue
│   ├── LICENSE.txt                      # Licence MIT
│   ├── README.txt                       # Informations d'installation
│   ├── build-installer.ps1              # Script PowerShell (Windows)
│   ├── build-installer.sh               # Script Bash (macOS/Linux)
│   ├── icon.ico                         # Icône de l'application (à créer)
│   ├── wizard-image.bmp                 # Image du wizard (à créer)
│   ├── wizard-small-image.bmp           # Petite image du wizard (à créer)
│   └── output/                          # Dossier de sortie (créé automatiquement)
└── src-tauri/
    └── target/
        └── release/
            └── z-cleaner.exe            # Exécutable compilé par Tauri
```

## 🚀 Étape 1 : Installer Inno Setup

### Sur Windows

1. Téléchargez Inno Setup 6 depuis : https://jrsoftware.org/isdl.php
2. Exécutez l'installeur
3. Acceptez les conditions de licence
4. Choisissez le dossier d'installation (par défaut : `C:\Program Files (x86)\Inno Setup 6`)
5. Terminez l'installation

### Sur macOS

```bash
# Installer Wine via Homebrew
brew install wine

# Installer Inno Setup via Wine (optionnel)
# Téléchargez l'installeur Windows d'Inno Setup et exécutez-le avec Wine
```

### Sur Linux

```bash
# Installer Wine
sudo apt-get install wine wine32 wine64

# Installer Inno Setup via Wine (optionnel)
```

## 🎨 Étape 2 : Préparer les Ressources Graphiques

### Créer l'icône (icon.ico)

1. Créez une image 256x256 pixels (PNG ou JPG)
2. Convertissez-la en ICO avec un outil en ligne :
   - https://convertio.co/png-ico/
   - https://icoconvert.com/
3. Sauvegardez le fichier sous `installer/icon.ico`

### Créer les images du Wizard

**wizard-image.bmp** (164x314 pixels) :
- Image de bienvenue affichée à gauche du wizard
- Format : BMP 24-bit
- Créez une image avec le logo Z-Cleaner et un design professionnel

**wizard-small-image.bmp** (55x55 pixels) :
- Petite icône affichée en haut du wizard
- Format : BMP 24-bit

**Outils recommandés :**
- Photoshop
- GIMP (gratuit) : https://www.gimp.org/
- Canva : https://www.canva.com/

## 🔨 Étape 3 : Compiler Tauri

Avant de créer l'installeur, compilez Tauri en mode release :

```bash
# Compiler Tauri
npm run tauri build

# Cela créera : src-tauri/target/release/z-cleaner.exe
```

## 📦 Étape 4 : Créer l'Installeur

### Option 1 : Utiliser le Script PowerShell (Windows - Recommandé)

```powershell
# Ouvrir PowerShell en tant qu'administrateur
# Naviguer vers le dossier du projet
cd C:\Users\YourUser\Documents\Dev\z-cleaner-2

# Exécuter le script
.\installer\build-installer.ps1

# Ou avec un chemin personnalisé pour Inno Setup
.\installer\build-installer.ps1 -InnoSetupPath "C:\Program Files (x86)\Inno Setup 6"
```

### Option 2 : Utiliser Inno Setup GUI (Windows)

1. Ouvrez Inno Setup Compiler
2. Allez dans `File` → `Open`
3. Sélectionnez `installer/z-cleaner-installer.iss`
4. Cliquez sur `Compile`
5. L'installeur sera créé dans `installer/output/`

### Option 3 : Utiliser le Script Bash (macOS/Linux)

```bash
# Rendre le script exécutable
chmod +x installer/build-installer.sh

# Exécuter le script
./installer/build-installer.sh
```

## 📝 Étape 5 : Automatiser avec npm

Ajoutez des commandes à votre `package.json` :

```json
{
  "scripts": {
    "tauri": "tauri",
    "tauri:build": "tauri build",
    "build:installer": "powershell -ExecutionPolicy Bypass -File installer/build-installer.ps1",
    "build:all": "npm run tauri:build && npm run build:installer",
    "build:all:unix": "npm run tauri:build && ./installer/build-installer.sh"
  }
}
```

### Utilisation

**Sur Windows :**
```bash
# Compiler Tauri et créer l'installeur
npm run build:all
```

**Sur macOS/Linux :**
```bash
# Compiler Tauri et créer l'installeur
npm run build:all:unix
```

## 🎯 Personnaliser l'Installeur

### Modifier le fichier .iss

Ouvrez `installer/z-cleaner-installer.iss` et modifiez :

```ini
; Informations de base
#define MyAppName "Z-Cleaner"
#define MyAppVersion "1.0.0"
#define MyAppPublisher "Votre Entreprise"
#define MyAppURL "https://votre-site.com"
#define MyAppDescription "Votre description"

; Dossier d'installation par défaut
DefaultDirName={autopf}\{#MyAppName}

; Fichiers à inclure
Source: "installer\icon.ico"; DestDir: "{app}"; Flags: ignoreversion
```

### Modifier les Textes

- **EULA_FR.txt** : Contrat de licence
- **WELCOME_FR.txt** : Message de bienvenue
- **LICENSE.txt** : Licence du logiciel
- **README.txt** : Instructions d'utilisation

## 🔍 Vérifier l'Installeur

Après la compilation, vérifiez que :

1. ✅ Le fichier `Z-Cleaner-Setup-1.0.0.exe` existe dans `installer/output/`
2. ✅ L'installeur se lance sans erreur
3. ✅ Toutes les étapes du wizard s'affichent correctement
4. ✅ L'application s'installe dans le bon dossier
5. ✅ Les raccourcis sont créés correctement
6. ✅ L'application se lance après l'installation

## 🐛 Dépannage

### Erreur : "Inno Setup n'est pas trouvé"

**Solution :**
- Vérifiez que Inno Setup 6 est installé
- Vérifiez le chemin d'installation (par défaut : `C:\Program Files (x86)\Inno Setup 6`)
- Modifiez le chemin dans le script PowerShell si nécessaire

### Erreur : "Fichier z-cleaner.exe non trouvé"

**Solution :**
- Compilez d'abord Tauri : `npm run tauri build`
- Vérifiez que le fichier existe dans `src-tauri/target/release/z-cleaner.exe`

### Erreur : "Impossible de créer le fichier de sortie"

**Solution :**
- Vérifiez que le dossier `installer/output/` est accessible
- Vérifiez les permissions du dossier
- Fermez tout fichier .exe ouvert dans ce dossier

### L'installeur ne démarre pas

**Solution :**
- Vérifiez que l'icône `installer/icon.ico` existe
- Vérifiez que les images du wizard existent
- Compilez à nouveau l'installeur

## 📊 Résultat Final

Après la compilation, vous aurez :

```
installer/output/
└── Z-Cleaner-Setup-1.0.0.exe (≈ 50-100 MB)
```

Cet exécutable peut être :
- ✅ Distribué à vos utilisateurs
- ✅ Hébergé sur un serveur de téléchargement
- ✅ Publié sur GitHub Releases
- ✅ Intégré dans un système de mise à jour automatique

## 🔄 Mise à Jour de l'Installeur

Pour mettre à jour l'installeur :

1. Modifiez la version dans `z-cleaner-installer.iss` :
   ```ini
   #define MyAppVersion "1.1.0"
   ```

2. Compilez Tauri :
   ```bash
   npm run tauri build
   ```

3. Créez le nouvel installeur :
   ```bash
   npm run build:installer
   ```

## 📚 Ressources

- **Inno Setup Documentation** : https://jrsoftware.org/ishelp/
- **Inno Setup Scripting** : https://jrsoftware.org/isinfo.php
- **Tauri Documentation** : https://tauri.app/
- **Windows Installer Best Practices** : https://docs.microsoft.com/en-us/windows/win32/msi/installer-best-practices

## ✅ Checklist Finale

Avant de distribuer l'installeur :

- [ ] Inno Setup 6 est installé
- [ ] Tauri est compilé en mode release
- [ ] L'icône `icon.ico` existe
- [ ] Les images du wizard existent
- [ ] Les fichiers EULA et LICENSE sont à jour
- [ ] Le script PowerShell s'exécute sans erreur
- [ ] L'installeur crée les raccourcis correctement
- [ ] L'application se lance après l'installation
- [ ] La désinstallation fonctionne correctement
- [ ] Le numéro de version est à jour

---

**Besoin d'aide ?** Consultez la documentation Inno Setup ou contactez le support.
