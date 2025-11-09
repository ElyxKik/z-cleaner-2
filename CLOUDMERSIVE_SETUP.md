# Configuration Cloudmersive Virus Scan API

## ✅ Vérification de la Configuration

### 1. Fichier `.env` Créé ✓

Le fichier `.env` a été créé à la racine du projet avec:
```
CLOUDMERSIVE_API_KEY=cbd9fc03-74c9-4234-8cc8-dbd9a343ba30
```

### 2. Dépendances Ajoutées ✓

Les dépendances suivantes ont été ajoutées à `Cargo.toml`:
```toml
reqwest = { version = "0.11", features = ["multipart", "json"] }
dotenv = "0.15"
```

### 3. Code Rust Intégré ✓

**Fichier:** `src-tauri/src/commands/virus_scanner.rs`
- ✅ Fonction `scan_file_with_cloudmersive()` - Scanne avec l'API
- ✅ Fonction `scan_file_and_report()` - Retourne un rapport
- ✅ Filtrage des faux positifs
- ✅ Gestion des erreurs

**Fichier:** `src-tauri/src/main.rs`
- ✅ Chargement de `.env` au démarrage
- ✅ Commande Tauri: `scan_file_with_cloudmersive_cmd`

## 🚀 Utilisation

### Via Frontend (React/TypeScript)

```typescript
import { invoke } from '@tauri-apps/api/tauri';

async function scanFileForVirus(filePath: string) {
  try {
    const result = await invoke('scan_file_with_cloudmersive_cmd', {
      filePath: filePath
    });
    
    console.log('Scan Result:', result);
    
    if (result.is_clean) {
      console.log('✅ File is clean:', result.reason);
    } else {
      console.log('⚠️ Threat detected:', result.reason);
    }
  } catch (error) {
    console.error('Scan failed:', error);
  }
}

// Utilisation
scanFileForVirus('/path/to/file.pdf');
```

### Via Rust (Backend)

```rust
use crate::commands::virus_scanner::scan_file_and_report;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    
    match scan_file_and_report("test.pdf").await {
        Ok(report) => {
            println!("Path: {}", report.path);
            println!("Clean: {}", report.is_clean);
            println!("Reason: {}", report.reason);
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

## 🧪 Tests

### Exécuter les tests

```bash
# Tests unitaires
cargo test --lib commands::virus_scanner

# Tests d'intégration
cargo test --test virus_scan_test

# Tous les tests
cargo test
```

### Résultats Attendus

```
test test_env_variables_loaded ... ok
test test_env_file_exists ... ok
test test_cloudmersive_api_connectivity ... ok
```

## 📊 Structure de Réponse

### MalwareScanReport

```rust
pub struct MalwareScanReport {
    pub path: String,              // Chemin du fichier
    pub is_clean: bool,            // true = propre, false = infecté
    pub threat_type: String,       // "none" ou "virus"
    pub threat_level: String,      // "safe" ou "critical"
    pub reason: String,            // Message détaillé
    pub file_size: u64,            // Taille en bytes
}
```

### Exemple de Réponse

```json
{
  "path": "/Users/user/Downloads/document.pdf",
  "is_clean": true,
  "threat_type": "none",
  "threat_level": "safe",
  "reason": "✅ File is clean - No viruses detected",
  "file_size": 1024000
}
```

## 🔒 Sécurité

### Bonnes Pratiques Implémentées

✅ **Clé API dans `.env`** (non versionné)
- Le fichier `.env` est dans `.gitignore`
- Jamais commité dans le repo

✅ **Chargement au Démarrage**
- `dotenv::dotenv().ok()` appelé dans `main()`
- Variables disponibles pour toutes les commandes

✅ **Gestion des Erreurs**
- Vérification de l'existence du fichier
- Vérification de la clé API
- Gestion des erreurs réseau

✅ **Filtrage des Faux Positifs**
- Répertoires système sûrs ignorés
- Extensions sûres reconnues
- Noms de fichiers système sûrs

## 🐛 Dépannage

### Erreur: "CLOUDMERSIVE_API_KEY not set"

**Solution:**
1. Vérifier que `.env` existe
2. Vérifier que la clé est présente
3. Redémarrer l'application

```bash
cat .env | grep CLOUDMERSIVE_API_KEY
```

### Erreur: "API request failed"

**Causes possibles:**
- Clé API invalide
- Limite de scans dépassée (50/mois gratuit)
- Pas de connexion Internet
- Fichier trop volumineux

**Solution:**
- Vérifier la clé API sur https://www.cloudmersive.com
- Vérifier la connexion Internet
- Vérifier la taille du fichier

### Erreur: "File not found"

**Solution:**
- Vérifier que le chemin est correct
- Vérifier que le fichier existe
- Utiliser un chemin absolu

## 📈 Prochaines Étapes

- [ ] Ajouter un cache des résultats
- [ ] Implémenter une quarantaine
- [ ] Ajouter des notifications
- [ ] Intégrer d'autres APIs antivirus

## 📚 Ressources

- [Cloudmersive API Docs](https://www.cloudmersive.com/virus-scan-api)
- [Reqwest Documentation](https://docs.rs/reqwest/)
- [Dotenv Documentation](https://docs.rs/dotenv/)
