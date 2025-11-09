# Configuration des Icônes Z-Cleaner

## 📋 Vue d'ensemble

Les icônes de Z-Cleaner sont générées à partir du logo PNG fourni et utilisées dans deux contextes:
1. **Icônes Tauri** - Pour l'application runtime (fenêtre, barre des tâches, etc.)
2. **Icônes Installeur** - Pour l'installeur Inno Setup

## 🎨 Logo Source

**Fichier:** `ChatGPT Image 3 sept. 2025, 20_01_25.png`

Logo: "Z" blanc avec balai sur fond bleu (#3F7FFF)

## 📁 Icônes Tauri

### Localisation
```
src-tauri/icons/
├── icon.ico                    # Icône Windows (.ico)
├── icon.icns                   # Icône macOS (.icns)
├── icon.png                    # Icône PNG (164x164)
├── 32x32.png                   # Petite icône
├── 128x128.png                 # Icône moyenne
├── 128x128@2x.png              # Icône haute résolution
├── StoreLogo.png               # Logo Microsoft Store
├── Square30x30Logo.png         # Icône 30x30
├── Square44x44Logo.png         # Icône 44x44
├── Square71x71Logo.png         # Icône 71x71
├── Square89x89Logo.png         # Icône 89x89
├── Square107x107Logo.png       # Icône 107x107
├── Square142x142Logo.png       # Icône 142x142
├── Square150x150Logo.png       # Icône 150x150
├── Square284x284Logo.png       # Icône 284x284
└── Square310x310Logo.png       # Icône 310x310
```

### Utilisation
- **icon.ico** - Icône Windows (EXE, barre des tâches)
- **icon.icns** - Icône macOS (DMG, barre des tâches)
- **icon.png** - Icône générique Linux
- **Square*.png** - Icônes Microsoft Store (UWP)

## 📦 Icônes Installeur

### Localisation
```
installer/
├── icon.ico                    # Icône 256x256 (installeur)
├── wizard-image.bmp            # Image wizard 164x314
└── wizard-small-image.bmp      # Petite image 55x55
```

### Utilisation
- **icon.ico** - Icône de l'installeur Inno Setup
- **wizard-image.bmp** - Image principale du wizard
- **wizard-small-image.bmp** - Petite icône du wizard

## 🔄 Régénérer les Icônes

### Régénérer les Icônes Tauri

```bash
# Générer toutes les icônes Tauri depuis le PNG
npm run icons:tauri

# Ou manuellement
npx @tauri-apps/cli icon "ChatGPT Image 3 sept. 2025, 20_01_25.png" -o src-tauri/icons
```

### Régénérer les Icônes Installeur

```bash
# Générer les ressources de l'installeur
npm run generate:assets

# Ou manuellement
python3 generate-installer-assets-from-logo.py
```

### Régénérer Toutes les Icônes

```bash
# Générer toutes les icônes (Tauri + Installeur)
npm run icons:tauri && npm run generate:assets
```

## 🚀 Workflow Complet

### 1. Générer les Icônes
```bash
npm run icons:tauri
npm run generate:assets
```

### 2. Recompiler l'Application
```bash
npm run dev        # Mode développement
npm run build      # Build production
```

### 3. Vérifier les Icônes

**En développement:**
- Lancer `npm run dev`
- Vérifier l'icône de la fenêtre Tauri
- Vérifier l'icône dans la barre des tâches

**En production:**
- Compiler: `npm run build`
- Vérifier l'icône du `.exe` dans `src-tauri/target/release/`
- Créer l'installeur: `npm run build:installer`
- Vérifier l'icône de l'installeur

## 📊 Commandes npm

### Icônes
```bash
npm run icons:tauri      # Régénérer les icônes Tauri
npm run generate:assets  # Régénérer les icônes installeur
```

### Build
```bash
npm run dev              # Mode développement
npm run build            # Build production
npm run build:installer  # Créer l'installeur (Windows)
npm run build:all        # Build complet + icônes + installeur
```

## 🎯 Build Complet (Recommandé)

```bash
# Génère les icônes, compile Tauri et crée l'installeur
npm run build:all
```

Cela exécute:
1. `npm run icons:tauri` - Régénère les icônes Tauri
2. `npm run generate:assets` - Régénère les icônes installeur
3. `npm run build` - Compile Tauri
4. `npm run build:installer` - Crée l'installeur (Windows)

## 🐛 Dépannage

### L'icône n'a pas changé après la régénération

**Solutions:**
1. Vider le cache Windows:
   ```bash
   # Redémarrer l'explorateur Windows
   taskkill /F /IM explorer.exe
   start explorer.exe
   ```

2. Changer le nom du binaire (Windows met parfois en cache les icônes)

3. Supprimer les fichiers générés et régénérer:
   ```bash
   rm -rf src-tauri/icons/*
   npm run icons:tauri
   ```

### Erreur: "Fichier PNG non trouvé"

**Solution:**
- Vérifiez que le fichier `ChatGPT Image 3 sept. 2025, 20_01_25.png` existe dans le dossier racine
- Vérifiez le chemin exact du fichier

### Les icônes Tauri ne sont pas mises à jour

**Solution:**
1. Supprimez les anciens fichiers:
   ```bash
   rm -rf src-tauri/icons/*
   ```

2. Régénérez les icônes:
   ```bash
   npm run icons:tauri
   ```

3. Recompilez:
   ```bash
   npm run build
   ```

## 📝 Modifier le Logo

Pour utiliser un nouveau logo:

1. Remplacez le fichier `ChatGPT Image 3 sept. 2025, 20_01_25.png` par votre nouveau logo

2. Régénérez les icônes:
   ```bash
   npm run icons:tauri
   npm run generate:assets
   ```

3. Recompilez:
   ```bash
   npm run build
   ```

## 📚 Ressources

- **Tauri Icons**: https://tauri.app/en/v1/guides/features/icons/
- **Inno Setup Icons**: https://jrsoftware.org/ishelp/
- **Windows Icon Format**: https://en.wikipedia.org/wiki/ICO_(file_format)

## ✅ Checklist

- [ ] Logo PNG fourni: `ChatGPT Image 3 sept. 2025, 20_01_25.png`
- [ ] Icônes Tauri générées: `src-tauri/icons/`
- [ ] Icônes installeur générées: `installer/`
- [ ] Application compilée: `npm run build`
- [ ] Icône de la fenêtre correcte
- [ ] Icône de la barre des tâches correcte
- [ ] Icône de l'EXE correcte
- [ ] Icône de l'installeur correcte

---

**Besoin d'aide ?** Exécutez `npm run icons:tauri && npm run generate:assets` pour régénérer toutes les icônes.
