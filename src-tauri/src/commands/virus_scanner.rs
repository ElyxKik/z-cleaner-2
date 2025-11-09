use reqwest::multipart;
use serde::{Deserialize, Serialize};
use std::env;
use std::path::Path;

/// Répertoires système sûrs à ignorer
const SAFE_SYSTEM_DIRS: &[&str] = &[
    ".trae/extensions",           // VS Code extensions
    ".vscode/extensions",         // VS Code extensions
    "node_modules",               // Node.js packages
    ".cargo",                     // Rust packages
    ".npm",                       // NPM cache
    ".gem",                       // Ruby gems
    ".m2",                        // Maven cache
    ".gradle",                    // Gradle cache
    "Library/Application Support", // macOS app data
    "AppData/Local/Programs",     // Windows programs
    "Program Files",              // Windows programs
    "/usr/local",                 // macOS/Linux system
    "/opt",                       // Linux system
    "typeshed",                   // Python type stubs
    "site-packages",              // Python packages
    "dist-packages",              // Python packages
];

/// Extensions de fichiers sûres
const SAFE_EXTENSIONS: &[&str] = &[
    ".pyi",   // Python type stub
    ".py",    // Python source
    ".js",    // JavaScript (dans node_modules c'est OK)
    ".ts",    // TypeScript
    ".json",  // JSON
    ".md",    // Markdown
    ".txt",   // Text
    ".yml",   // YAML
    ".yaml",  // YAML
];

/// Noms de fichiers système sûrs
const SAFE_FILENAMES: &[&str] = &[
    "cmd.pyi",      // Python type stub
    "cmd.py",       // Python module
    "powershell.pyi", // Python type stub
    "bash",         // Shell
    "sh",           // Shell
];

/// Réponse de l'API Cloudmersive Virus Scan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirusScanResult {
    #[serde(rename = "CleanResult")]
    pub clean_result: bool,
    
    #[serde(rename = "FoundViruses")]
    pub found_viruses: Option<Vec<String>>,
    
    #[serde(rename = "FileName")]
    pub file_name: Option<String>,
    
    #[serde(rename = "FileSize")]
    pub file_size: Option<i64>,
}

/// Structure pour retourner le résultat du scan au frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MalwareScanReport {
    pub path: String,
    pub is_clean: bool,
    pub threat_type: String,
    pub threat_level: String,
    pub reason: String,
    pub file_size: u64,
}

/// Scanne un fichier avec l'API Cloudmersive Virus Scan
pub async fn scan_file_with_cloudmersive(file_path: &str) -> Result<VirusScanResult, String> {
    println!("🔍 [VIRUS_SCANNER] Starting Cloudmersive API scan for: {}", file_path);
    
    // Récupérer la clé API depuis les variables d'environnement
    let api_key = env::var("CLOUDMERSIVE_API_KEY")
        .map_err(|_| {
            eprintln!("❌ [VIRUS_SCANNER] CLOUDMERSIVE_API_KEY environment variable not set");
            "CLOUDMERSIVE_API_KEY environment variable not set".to_string()
        })?;
    
    println!("✓ [VIRUS_SCANNER] API Key loaded: {}...{}", 
        &api_key[..8], 
        &api_key[api_key.len()-4..]
    );

    // Vérifier que le fichier existe
    if !Path::new(file_path).exists() {
        eprintln!("❌ [VIRUS_SCANNER] File not found: {}", file_path);
        return Err(format!("File not found: {}", file_path));
    }
    
    println!("✓ [VIRUS_SCANNER] File exists: {}", file_path);

    // Lire le fichier
    let file_bytes = tokio::fs::read(file_path)
        .await
        .map_err(|e| {
            eprintln!("❌ [VIRUS_SCANNER] Failed to read file: {}", e);
            format!("Failed to read file: {}", e)
        })?;
    
    println!("✓ [VIRUS_SCANNER] File read successfully: {} bytes", file_bytes.len());

    // Créer le client HTTP
    let client = reqwest::Client::new();

    // Extraire le nom du fichier
    let file_name = Path::new(file_path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("file")
        .to_string();

    // Créer le formulaire multipart
    let form = multipart::Form::new()
        .part("file", multipart::Part::bytes(file_bytes).file_name(file_name.clone()));

    println!("📤 [VIRUS_SCANNER] Sending request to Cloudmersive API...");
    println!("   Endpoint: https://api.cloudmersive.com/virus/scan/file");
    println!("   File: {}", file_name);

    // Envoyer la requête POST à l'API Cloudmersive
    let response = client
        .post("https://api.cloudmersive.com/virus/scan/file")
        .header("Apikey", &api_key)
        .multipart(form)
        .send()
        .await
        .map_err(|e| {
            eprintln!("❌ [VIRUS_SCANNER] API request failed: {}", e);
            format!("API request failed: {}", e)
        })?;

    let status = response.status();
    println!("📥 [VIRUS_SCANNER] API Response Status: {}", status);

    // Parser la réponse JSON
    let scan_result: VirusScanResult = response
        .json()
        .await
        .map_err(|e| {
            eprintln!("❌ [VIRUS_SCANNER] Failed to parse API response: {}", e);
            format!("Failed to parse API response: {}", e)
        })?;

    println!("✓ [VIRUS_SCANNER] Scan Result: clean={}, viruses={:?}", 
        scan_result.clean_result,
        scan_result.found_viruses
    );

    Ok(scan_result)
}

/// Vérifie si un fichier est dans un répertoire système sûr
fn is_in_safe_directory(file_path: &str) -> bool {
    let path_lower = file_path.to_lowercase();
    SAFE_SYSTEM_DIRS.iter().any(|dir| path_lower.contains(&dir.to_lowercase()))
}

/// Vérifie si un fichier a une extension sûre
fn has_safe_extension(file_path: &str) -> bool {
    SAFE_EXTENSIONS.iter().any(|ext| file_path.ends_with(ext))
}

/// Vérifie si c'est un nom de fichier système sûr
fn is_safe_filename(file_path: &str) -> bool {
    let file_name = Path::new(file_path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("")
        .to_lowercase();
    
    SAFE_FILENAMES.iter().any(|name| file_name.contains(&name.to_lowercase()))
}

/// Filtre les faux positifs de l'API Cloudmersive
fn is_false_positive(file_path: &str) -> bool {
    // Si c'est dans un répertoire système sûr ET a une extension sûre
    if is_in_safe_directory(file_path) && has_safe_extension(file_path) {
        return true;
    }
    
    // Si c'est un nom de fichier système sûr
    if is_safe_filename(file_path) {
        return true;
    }
    
    false
}

/// Scanne un fichier et retourne un rapport formaté
pub async fn scan_file_and_report(file_path: &str) -> Result<MalwareScanReport, String> {
    println!("\n📋 [VIRUS_REPORT] Starting scan report for: {}", file_path);
    
    // Obtenir la taille du fichier
    let metadata = tokio::fs::metadata(file_path)
        .await
        .map_err(|e| format!("Failed to get file metadata: {}", e))?;

    let file_size = metadata.len();
    println!("📊 [VIRUS_REPORT] File size: {} bytes", file_size);

    // Vérifier les faux positifs connus avant de scanner
    if is_false_positive(file_path) {
        println!("⚠️  [VIRUS_REPORT] File is a known false positive (system file)");
        return Ok(MalwareScanReport {
            path: file_path.to_string(),
            is_clean: true,
            threat_type: "none".to_string(),
            threat_level: "safe".to_string(),
            reason: "✅ File is safe (system file)".to_string(),
            file_size,
        });
    }

    // Scanne avec l'API Cloudmersive
    println!("🌐 [VIRUS_REPORT] Calling Cloudmersive API...");
    let scan_result = scan_file_with_cloudmersive(file_path).await?;

    // Créer le rapport
    let report = if scan_result.clean_result {
        MalwareScanReport {
            path: file_path.to_string(),
            is_clean: true,
            threat_type: "none".to_string(),
            threat_level: "safe".to_string(),
            reason: "✅ File is clean - No viruses detected".to_string(),
            file_size,
        }
    } else {
        // Vérifier si c'est un faux positif même après le scan
        if is_false_positive(file_path) {
            MalwareScanReport {
                path: file_path.to_string(),
                is_clean: true,
                threat_type: "none".to_string(),
                threat_level: "safe".to_string(),
                reason: "✅ File is safe (false positive filtered)".to_string(),
                file_size,
            }
        } else {
            let viruses = scan_result
                .found_viruses
                .unwrap_or_default()
                .join(", ");
            
            MalwareScanReport {
                path: file_path.to_string(),
                is_clean: false,
                threat_type: "virus".to_string(),
                threat_level: "critical".to_string(),
                reason: format!("⚠️ Virus detected: {}", viruses),
                file_size,
            }
        }
    };

    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_scan_file_not_found() {
        let result = scan_file_with_cloudmersive("nonexistent.pdf").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_api_key_missing() {
        // Temporairement supprimer la clé API
        env::remove_var("CLOUDMERSIVE_API_KEY");
        let result = scan_file_with_cloudmersive("test.pdf").await;
        assert!(result.is_err());
    }
}
