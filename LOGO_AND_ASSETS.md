# Logo et Ressources Graphiques Z-Cleaner

## 📋 Vue d'ensemble

Les ressources graphiques de Z-Cleaner sont générées automatiquement à partir du logo PNG fourni.

## 🎨 Logo Source

**Fichier:** `ChatGPT Image 3 sept. 2025, 20_01_25.png`

Le logo est un "Z" blanc avec un balai sur fond bleu (#3F7FFF).

## 📦 Ressources Générées

### 1. **icon.ico** (256x256)
- Format: ICO (Windows Icon)
- Utilisation: Icône de l'application dans l'explorateur Windows
- Fond: Bleu (#3F7FFF)
- Logo: Centré et redimensionné à 220x220

### 2. **wizard-image.bmp** (164x314)
- Format: BMP (Bitmap)
- Utilisation: Image principale de l'assistant d'installation Inno Setup
- Fond: Dégradé bleu (du bleu clair au bleu foncé)
- Logo: Centré et redimensionné à 140x140

### 3. **wizard-small-image.bmp** (55x55)
- Format: BMP (Bitmap)
- Utilisation: Petite icône du wizard Inno Setup
- Fond: Bleu (#3F7FFF)
- Logo: Centré et redimensionné à 50x50

## 🔄 Régénérer les Ressources

### Automatiquement (Recommandé)

```bash
# Générer les ressources
npm run generate:assets

# Ou lors du build complet
npm run build:all
```

### Manuellement

```bash
# Exécuter le script Python directement
python3 generate-installer-assets-from-logo.py
```

## 📁 Structure des Fichiers

```
z-cleaner-2/
├── ChatGPT Image 3 sept. 2025, 20_01_25.png  # Logo source
├── generate-installer-assets-from-logo.py     # Script de génération
├── installer/
│   ├── icon.ico                               # Icône 256x256
│   ├── wizard-image.bmp                       # Image wizard 164x314
│   ├── wizard-small-image.bmp                 # Petite image 55x55
│   ├── z-cleaner-installer.iss                # Script Inno Setup
│   ├── EULA_FR.txt
│   ├── WELCOME_FR.txt
│   ├── LICENSE.txt
│   ├── README.txt
│   ├── build-installer.ps1
│   └── build-installer.sh
└── package.json
```

## 🛠️ Prérequis

### Python 3
```bash
# Vérifier que Python 3 est installé
python3 --version

# Installer Pillow (PIL) si nécessaire
pip3 install Pillow
```

## 📝 Commandes npm

### Générer les Ressources
```bash
npm run generate:assets
```

### Compiler Tauri
```bash
npm run build
```

### Générer les Ressources + Compiler Tauri + Créer l'Installeur
```bash
npm run build:all
```

### Créer l'Installeur (Windows uniquement)
```bash
npm run build:installer
```

## 🎯 Workflow Complet

### Sur Windows

```bash
# 1. Générer les ressources graphiques
npm run generate:assets

# 2. Compiler Tauri
npm run build

# 3. Créer l'installeur
npm run build:installer

# OU en une seule commande
npm run build:all
```

### Sur macOS/Linux

```bash
# 1. Générer les ressources graphiques
npm run generate:assets

# 2. Compiler Tauri
npm run build

# Note: L'installeur .exe ne peut être créé que sur Windows
```

## 🔍 Vérifier les Ressources

### Vérifier que les fichiers existent

```bash
ls -lh installer/icon.ico installer/wizard-*.bmp
```

### Afficher les informations des fichiers

```bash
# Sur macOS
file installer/icon.ico installer/wizard-*.bmp

# Sur Linux
file installer/icon.ico installer/wizard-*.bmp
```

## 🎨 Personnaliser les Ressources

### Modifier le Logo

1. Remplacez le fichier `ChatGPT Image 3 sept. 2025, 20_01_25.png` par votre nouveau logo
2. Exécutez `npm run generate:assets`
3. Les ressources seront régénérées automatiquement

### Modifier les Couleurs

Éditez `generate-installer-assets-from-logo.py` et modifiez les constantes:

```python
# Couleurs
BLUE = (63, 127, 255)      # Couleur principale
DARK_BLUE = (41, 98, 255)  # Couleur foncée
WHITE = (255, 255, 255)    # Couleur claire
```

### Modifier les Tailles

Éditez les fonctions dans `generate-installer-assets-from-logo.py`:

```python
def create_icon():
    # Redimensionner le logo
    logo = load_and_resize_logo(220)  # Changer 220 pour une autre taille
```

## 🐛 Dépannage

### Erreur: "Le fichier logo n'existe pas"

**Solution:**
- Vérifiez que le fichier `ChatGPT Image 3 sept. 2025, 20_01_25.png` existe dans le dossier racine
- Vérifiez le chemin exact du fichier

### Erreur: "ModuleNotFoundError: No module named 'PIL'"

**Solution:**
```bash
pip3 install Pillow
```

### Les ressources ne sont pas mises à jour

**Solution:**
- Supprimez les anciens fichiers:
  ```bash
  rm installer/icon.ico installer/wizard-*.bmp
  ```
- Régénérez les ressources:
  ```bash
  npm run generate:assets
  ```

## 📊 Résultat Final

Après l'exécution du script, vous aurez:

```
installer/
├── icon.ico (27 KB)
├── wizard-image.bmp (151 KB)
└── wizard-small-image.bmp (9.1 KB)
```

Ces fichiers sont prêts pour:
- ✅ L'installeur Inno Setup
- ✅ L'application Tauri
- ✅ La distribution Windows

## 📚 Ressources

- **Pillow Documentation**: https://pillow.readthedocs.io/
- **Inno Setup**: https://jrsoftware.org/ishelp/
- **Tauri**: https://tauri.app/

---

**Besoin d'aide ?** Consultez la documentation ou exécutez `npm run generate:assets` pour régénérer les ressources.
