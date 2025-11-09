# Intégration API Cloudmersive Virus Scan

## 📋 Objectif

Intégrer l'API Cloudmersive Virus Scan dans Z-Cleaner pour analyser des fichiers contre les virus avant leur traitement.

## 🔧 Configuration

### 1. Obtenir une clé API

1. Visitez [Cloudmersive Virus Scan API](https://www.cloudmersive.com/virus-scan-api)
2. Inscrivez-vous pour un compte gratuit (50 scans/mois)
3. Récupérez votre clé API

### 2. Configurer la variable d'environnement

Créez un fichier `.env` à la racine du projet:

```bash
CLOUDMERSIVE_API_KEY=votre_clé_api_ici
```

Ou copiez `.env.example` et remplissez la clé:

```bash
cp .env.example .env
```

## 📦 Dépendances Rust

Les dépendances suivantes sont déjà configurées dans `Cargo.toml`:

```toml
reqwest = { version = "0.11", features = ["multipart", "json"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
dotenv = "0.15"
sha2 = "0.10"
```

## 🚀 Utilisation

### Via Tauri Command (Frontend)

```typescript
// Dans un composant React
import { invoke } from '@tauri-apps/api/tauri';

async function scanFile(filePath: string) {
  try {
    const result = await invoke('scan_file_with_cloudmersive_cmd', {
      filePath: filePath
    });
    
    if (result.is_clean) {
      console.log('✅ Fichier propre:', result.reason);
    } else {
      console.log('⚠️ Virus détecté:', result.reason);
    }
  } catch (error) {
    console.error('Erreur du scan:', error);
  }
}
```

### Via Rust (Backend)

```rust
use crate::commands::virus_scanner::scan_file_and_report;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    
    match scan_file_and_report("test.pdf").await {
        Ok(report) => {
            println!("Rapport: {:?}", report);
        }
        Err(e) => {
            eprintln!("Erreur: {}", e);
        }
    }
}
```

## 📊 Structure de Réponse

### VirusScanResult (API Cloudmersive)

```json
{
  "CleanResult": true,
  "FoundViruses": null,
  "FileName": "document.pdf",
  "FileSize": 1024000
}
```

### MalwareScanReport (Notre format)

```rust
pub struct MalwareScanReport {
    pub path: String,
    pub is_clean: bool,
    pub threat_type: String,
    pub threat_level: String,
    pub reason: String,
    pub file_size: u64,
}
```

## 🔍 Niveaux de Menace

- **SAFE** - Fichier propre
- **CRITICAL** - Virus détecté

## 📝 Exemple Complet

```rust
use dotenv;
use crate::commands::virus_scanner::scan_file_and_report;

#[tokio::main]
async fn main() {
    // Charger les variables d'environnement
    dotenv::dotenv().ok();
    
    // Scanner un fichier
    let file_path = "test.pdf";
    
    match scan_file_and_report(file_path).await {
        Ok(report) => {
            println!("Fichier: {}", report.path);
            println!("Propre: {}", report.is_clean);
            println!("Type de menace: {}", report.threat_type);
            println!("Niveau: {}", report.threat_level);
            println!("Raison: {}", report.reason);
            println!("Taille: {} bytes", report.file_size);
        }
        Err(e) => {
            eprintln!("Erreur: {}", e);
        }
    }
}
```

## 🧪 Tests

Exécuter les tests unitaires:

```bash
cargo test --lib commands::virus_scanner
```

## ⚠️ Limitations

- **Limite gratuite**: 50 scans/mois
- **Taille max**: Dépend du plan Cloudmersive
- **Hash SHA256**: Calculé pour les fichiers < 10MB

## 🔐 Sécurité

- La clé API est stockée dans `.env` (non versionné)
- Les fichiers ne sont pas stockés sur les serveurs Cloudmersive
- Utilisation de HTTPS pour toutes les requêtes

## 📚 Ressources

- [Documentation Cloudmersive](https://www.cloudmersive.com/virus-scan-api)
- [Reqwest Documentation](https://docs.rs/reqwest/)
- [Tokio Documentation](https://tokio.rs/)

## 🐛 Dépannage

### Erreur: "CLOUDMERSIVE_API_KEY environment variable not set"

**Solution**: Assurez-vous que le fichier `.env` existe et contient la clé API.

```bash
echo "CLOUDMERSIVE_API_KEY=votre_clé" > .env
```

### Erreur: "API request failed"

**Solution**: Vérifiez que:
1. Votre clé API est valide
2. Vous avez des scans disponibles (limite gratuite: 50/mois)
3. Votre connexion Internet fonctionne

### Erreur: "File not found"

**Solution**: Vérifiez que le chemin du fichier est correct et que le fichier existe.

## 📈 Prochaines Étapes

- [ ] Ajouter un cache des résultats de scan
- [ ] Implémenter un système de quarantaine
- [ ] Ajouter des notifications en temps réel
- [ ] Intégrer d'autres APIs antivirus (VirusTotal, etc.)
