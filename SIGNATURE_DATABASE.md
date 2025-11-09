# Base de Données de Signatures de Malwares

## 📋 Vue d'ensemble

La base de données de signatures permet de détecter les malwares connus localement sans dépendre d'une API externe.

## 🏗️ Architecture

### Fichier: `src-tauri/src/commands/signature_db.rs`

**Structures:**
- `MalwareSignature` - Représente une signature de malware
- `SignatureDatabase` - Gère la collection de signatures

**Fonctions principales:**
- `SignatureDatabase::new()` - Crée la DB avec les signatures connues
- `find_by_sha256()` - Cherche par hash SHA256
- `find_by_md5()` - Cherche par hash MD5
- `find_by_name()` - Cherche par nom
- `scan_file_against_signatures()` - Scanne un fichier

## 📊 Signatures Incluses

### Ransomware
```rust
Ransomware.WannaCry
  SHA256: ed01ebfbc9eb5bbea545af4d01bf5f1071661840480cfd4b3b8f2bbb857e038e
  MD5: 84c82835a5d21bbcf75a61706d8ab549
  Threat Level: CRITICAL
```

### Trojans
```rust
Trojan.Win32.Generic
Trojan.Miner.Generic
Trojan.Generic
```

### Adware & PUP
```rust
Adware.PUP.Generic
```

### Spyware
```rust
Spyware.Generic
```

## 🔧 Utilisation

### 1. Créer une instance de la base de données

```rust
use crate::commands::signature_db::SignatureDatabase;

let db = SignatureDatabase::new();
println!("Loaded {} signatures", db.count());
```

### 2. Chercher une signature

**Par SHA256:**
```rust
if let Some(sig) = db.find_by_sha256("ed01ebfbc9eb5bbea545af4d01bf5f1071661840480cfd4b3b8f2bbb857e038e") {
    println!("Found: {}", sig.name);
}
```

**Par MD5:**
```rust
if let Some(sig) = db.find_by_md5("84c82835a5d21bbcf75a61706d8ab549") {
    println!("Found: {}", sig.name);
}
```

**Par nom:**
```rust
if let Some(sig) = db.find_by_name("Ransomware.WannaCry") {
    println!("Threat Level: {}", sig.threat_level);
}
```

### 3. Scanner un fichier

```rust
use crate::commands::signature_db::scan_file_against_signatures;

let db = SignatureDatabase::new();
match scan_file_against_signatures("/path/to/file.exe", &db) {
    Ok(Some(signature)) => {
        println!("⚠️ Malware detected: {}", signature.name);
    }
    Ok(None) => {
        println!("✓ File is clean");
    }
    Err(e) => {
        eprintln!("Error: {}", e);
    }
}
```

## 🔄 Intégration avec Malware Detector

Le `malware_detector.rs` utilise maintenant la base de données de signatures:

```rust
// Dans check_file_for_malware()
let db = SignatureDatabase::new();
if let Ok(Some(signature)) = scan_file_against_signatures(path, &db) {
    return Some(MalwareFile {
        path: path.to_string(),
        threat_level: signature.threat_level,
        threat_type: signature.threat_type,
        reason: format!("Signature match: {}", signature.name),
        size: metadata.len(),
        hash: calculate_file_hash(path),
        is_autostart: false,
    });
}
```

## ➕ Ajouter de Nouvelles Signatures

### Méthode 1: Ajouter dans `load_known_signatures()`

```rust
fn load_known_signatures(&mut self) {
    self.add_signature(MalwareSignature {
        name: "Trojan.NewMalware".to_string(),
        hash_sha256: Some("abc123...".to_string()),
        hash_md5: Some("def456...".to_string()),
        threat_level: "high".to_string(),
        threat_type: "trojan".to_string(),
        description: "New malware detected".to_string(),
    });
}
```

### Méthode 2: Ajouter dynamiquement

```rust
let mut db = SignatureDatabase::new();
db.add_signature(MalwareSignature {
    name: "Custom.Malware".to_string(),
    hash_sha256: Some("hash...".to_string()),
    hash_md5: None,
    threat_level: "critical".to_string(),
    threat_type: "virus".to_string(),
    description: "Custom malware".to_string(),
});
```

## 📈 Amélioration Future

### Charger depuis un fichier JSON

```rust
// signatures.json
{
  "signatures": [
    {
      "name": "Ransomware.WannaCry",
      "sha256": "ed01ebfbc9eb5bbea545af4d01bf5f1071661840480cfd4b3b8f2bbb857e038e",
      "md5": "84c82835a5d21bbcf75a61706d8ab549",
      "threat_level": "critical",
      "threat_type": "ransomware"
    }
  ]
}
```

### Synchroniser avec une API

```rust
// Télécharger les signatures depuis une source externe
async fn sync_signatures_from_api() -> Result<Vec<MalwareSignature>, String> {
    // Appel API pour obtenir les signatures à jour
}
```

## 🧪 Tests

Exécuter les tests:

```bash
cargo test --lib commands::signature_db
```

Résultats attendus:
```
test test_signature_database_creation ... ok
test test_find_signature_by_name ... ok
test test_find_signature_by_sha256 ... ok
```

## 📊 Logs

Les logs montrent l'utilisation de la base de données:

```
✓ [SIGNATURE_DB] Loaded 5 malware signatures
🔍 [SIGNATURE_DB] Scanning file against signature database: /path/to/file.exe
📊 [SIGNATURE_DB] File SHA256: ed01ebfbc9eb5bbea545af4d01bf5f1071661840480cfd4b3b8f2bbb857e038e
⚠️  [SIGNATURE_DB] MATCH FOUND: Ransomware.WannaCry
```

## 🔐 Sécurité

- ✅ Signatures stockées localement (pas de dépendance réseau)
- ✅ Hashes SHA256 pour une détection précise
- ✅ Niveaux de menace configurables
- ✅ Descriptions détaillées

## 📚 Ressources

- [YARA Rules](https://github.com/Yara-Rules/rules) - Règles de détection open-source
- [VirusShare](https://virusshare.com/) - Base de données de malwares
- [Malware Traffic Analysis](https://www.malware-traffic-analysis.net/) - Analyse de trafic malveillant
