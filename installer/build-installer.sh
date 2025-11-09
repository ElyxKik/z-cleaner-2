#!/bin/bash

# Script bash pour compiler l'installeur Z-Cleaner avec Inno Setup (via Wine sur macOS/Linux)
# Utilisation : ./build-installer.sh

set -e

echo "========================================"
echo "Z-Cleaner Installer Builder (macOS/Linux)"
echo "========================================"
echo ""

# Configuration
ISS_FILE="installer/z-cleaner-installer.iss"
OUTPUT_DIR="installer/output"

# Vérifier si le fichier .iss existe
if [ ! -f "$ISS_FILE" ]; then
    echo "❌ Erreur: Fichier $ISS_FILE non trouvé"
    exit 1
fi

echo "✓ Fichier de configuration trouvé: $ISS_FILE"
echo ""

# Créer le dossier de sortie s'il n'existe pas
mkdir -p "$OUTPUT_DIR"
echo "✓ Dossier de sortie: $OUTPUT_DIR"
echo ""

# Vérifier si Wine est installé (pour Inno Setup)
if ! command -v wine &> /dev/null; then
    echo "⚠️  Wine n'est pas installé"
    echo ""
    echo "Pour compiler l'installeur sur macOS/Linux, vous devez :"
    echo "1. Installer Wine : brew install wine (macOS) ou apt-get install wine (Linux)"
    echo "2. Installer Inno Setup via Wine"
    echo ""
    echo "OU"
    echo ""
    echo "Utilisez Windows ou une machine virtuelle Windows pour compiler l'installeur."
    echo ""
    echo "Note: L'installeur .exe ne peut être créé que sur Windows."
    exit 1
fi

echo "✓ Wine trouvé"
echo ""

# Vérifier si Inno Setup est installé
if ! command -v iscc &> /dev/null; then
    echo "❌ Inno Setup n'est pas trouvé"
    echo "Veuillez installer Inno Setup via Wine"
    exit 1
fi

echo "✓ Inno Setup trouvé"
echo ""

echo "Compilation de l'installeur en cours..."
echo ""

# Compiler avec Inno Setup
iscc "$ISS_FILE"

if [ $? -eq 0 ]; then
    echo ""
    echo "========================================"
    echo "✓ Compilation réussie !"
    echo "========================================"
    echo ""
    echo "L'installeur a été créé dans: $OUTPUT_DIR"
    
    # Afficher les fichiers créés
    if [ -d "$OUTPUT_DIR" ]; then
        echo ""
        echo "Fichiers créés:"
        ls -lh "$OUTPUT_DIR"/*.exe 2>/dev/null || echo "Aucun fichier .exe trouvé"
    fi
else
    echo ""
    echo "========================================"
    echo "❌ Erreur lors de la compilation"
    echo "========================================"
    exit 1
fi

echo ""
echo "Terminé !"
