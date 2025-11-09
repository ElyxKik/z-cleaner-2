/// Base de données locale de signatures de malwares
/// Utilise des hashes MD5/SHA256 et des patterns pour détecter les malwares connus

use std::collections::HashMap;
use sha2::{Sha256, Digest};
use std::fs;

/// Structure pour une signature de malware
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct MalwareSignature {
    pub name: String,
    pub hash_sha256: Option<String>,
    pub hash_md5: Option<String>,
    pub threat_level: String,
    pub threat_type: String,
    pub description: String,
}

/// Base de données de signatures
#[allow(dead_code)]
pub struct SignatureDatabase {
    signatures: HashMap<String, MalwareSignature>,
}

impl SignatureDatabase {
    /// Crée une nouvelle base de données avec les signatures connues
    #[allow(dead_code)]
    pub fn new() -> Self {
        let mut db = SignatureDatabase {
            signatures: HashMap::new(),
        };
        
        // Initialiser avec les signatures connues
        db.load_known_signatures();
        
        db
    }
    
    /// Charge les signatures malveillantes connues
    fn load_known_signatures(&mut self) {
        // Trojans Windows courants
        self.add_signature(MalwareSignature {
            name: "Trojan.Win32.Generic".to_string(),
            hash_sha256: None,
            hash_md5: None,
            threat_level: "critical".to_string(),
            threat_type: "trojan".to_string(),
            description: "Generic Windows trojan".to_string(),
        });
        
        // Ransomware courants
        self.add_signature(MalwareSignature {
            name: "Ransomware.WannaCry".to_string(),
            hash_sha256: Some("ed01ebfbc9eb5bbea545af4d01bf5f1071661840480cfd4b3b8f2bbb857e038e".to_string()),
            hash_md5: Some("84c82835a5d21bbcf75a61706d8ab549".to_string()),
            threat_level: "critical".to_string(),
            threat_type: "ransomware".to_string(),
            description: "WannaCry ransomware - CRITICAL".to_string(),
        });
        
        // Miners de crypto-monnaies
        self.add_signature(MalwareSignature {
            name: "Trojan.Miner.Generic".to_string(),
            hash_sha256: None,
            hash_md5: None,
            threat_level: "high".to_string(),
            threat_type: "miner".to_string(),
            description: "Cryptocurrency miner trojan".to_string(),
        });
        
        // Adware courant
        self.add_signature(MalwareSignature {
            name: "Adware.PUP.Generic".to_string(),
            hash_sha256: None,
            hash_md5: None,
            threat_level: "medium".to_string(),
            threat_type: "adware".to_string(),
            description: "Potentially Unwanted Program".to_string(),
        });
        
        // Spyware courant
        self.add_signature(MalwareSignature {
            name: "Spyware.Generic".to_string(),
            hash_sha256: None,
            hash_md5: None,
            threat_level: "high".to_string(),
            threat_type: "spyware".to_string(),
            description: "Generic spyware".to_string(),
        });
        
        println!("✓ [SIGNATURE_DB] Loaded {} malware signatures", self.signatures.len());
    }
    
    /// Ajoute une signature à la base de données
    pub fn add_signature(&mut self, sig: MalwareSignature) {
        self.signatures.insert(sig.name.clone(), sig);
    }
    
    /// Cherche une signature par hash SHA256
    #[allow(dead_code)]
    pub fn find_by_sha256(&self, hash: &str) -> Option<&MalwareSignature> {
        self.signatures.values().find(|sig| {
            sig.hash_sha256.as_ref().map_or(false, |h| h.eq_ignore_ascii_case(hash))
        })
    }
    
    /// Cherche une signature par hash MD5
    #[allow(dead_code)]
    pub fn find_by_md5(&self, hash: &str) -> Option<&MalwareSignature> {
        self.signatures.values().find(|sig| {
            sig.hash_md5.as_ref().map_or(false, |h| h.eq_ignore_ascii_case(hash))
        })
    }
    
    /// Cherche une signature par nom
    #[allow(dead_code)]
    pub fn find_by_name(&self, name: &str) -> Option<&MalwareSignature> {
        self.signatures.get(name)
    }
    
    /// Retourne toutes les signatures
    #[allow(dead_code)]
    pub fn get_all(&self) -> Vec<&MalwareSignature> {
        self.signatures.values().collect()
    }
    
    /// Retourne le nombre de signatures
    #[allow(dead_code)]
    pub fn count(&self) -> usize {
        self.signatures.len()
    }
}

/// Calcule le hash SHA256 d'un fichier
#[allow(dead_code)]
pub fn calculate_file_sha256(path: &str) -> Result<String, String> {
    let file_bytes = fs::read(path)
        .map_err(|e| format!("Failed to read file: {}", e))?;
    
    let mut hasher = Sha256::new();
    hasher.update(&file_bytes);
    let result = hasher.finalize();
    
    Ok(format!("{:x}", result))
}

/// Scanne un fichier contre la base de données de signatures
#[allow(dead_code)]
pub fn scan_file_against_signatures(file_path: &str, db: &SignatureDatabase) -> Result<Option<MalwareSignature>, String> {
    println!("🔍 [SIGNATURE_DB] Scanning file against signature database: {}", file_path);
    
    // Calculer le hash SHA256
    let file_hash = calculate_file_sha256(file_path)?;
    println!("📊 [SIGNATURE_DB] File SHA256: {}", file_hash);
    
    // Chercher dans la base de données
    if let Some(signature) = db.find_by_sha256(&file_hash) {
        println!("⚠️  [SIGNATURE_DB] MATCH FOUND: {}", signature.name);
        return Ok(Some(signature.clone()));
    }
    
    println!("✓ [SIGNATURE_DB] No signature match found");
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_signature_database_creation() {
        let db = SignatureDatabase::new();
        assert!(db.count() > 0);
        println!("✓ Signature database created with {} signatures", db.count());
    }
    
    #[test]
    fn test_find_signature_by_name() {
        let db = SignatureDatabase::new();
        let sig = db.find_by_name("Ransomware.WannaCry");
        assert!(sig.is_some());
        assert_eq!(sig.unwrap().threat_level, "critical");
    }
    
    #[test]
    fn test_find_signature_by_sha256() {
        let db = SignatureDatabase::new();
        let hash = "ed01ebfbc9eb5bbea545af4d01bf5f1071661840480cfd4b3b8f2bbb857e038e";
        let sig = db.find_by_sha256(hash);
        assert!(sig.is_some());
        assert_eq!(sig.unwrap().name, "Ransomware.WannaCry");
    }
}
