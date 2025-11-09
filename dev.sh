#!/bin/bash

# Script pour lancer Z-Cleaner en mode développement

echo "🚀 Démarrage de Z-Cleaner..."

# Charger l'environnement Cargo
source $HOME/.cargo/env

# Lancer Tauri (qui lancera automatiquement Vite via beforeDevCommand)
npm run dev:tauri
