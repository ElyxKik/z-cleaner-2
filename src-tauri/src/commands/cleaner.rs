use crate::models::{CleanResult, CleanScanResult, CleanScanResults, CleanableFile};
use chrono::Utc;
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;

/// Scanne l'ordinateur pour trouver les fichiers à nettoyer avec progression
pub async fn scan_for_cleaning_with_progress(window: tauri::Window) -> Result<CleanScanResults, String> {
    let mut categories = Vec::new();
    let mut total_files = 0u32;
    let mut total_space = 0u64;
    let mut all_files = Vec::new();

    // Scan Temp Files
    let _ = window.emit("clean-scan-progress", serde_json::json!({
        "category": "temp",
        "status": "scanning"
    }));
    let (temp_files, temp_space, temp_file_list) = scan_temp_files_with_details();
    total_files += temp_files;
    total_space += temp_space;
    all_files.extend(temp_file_list);
    categories.push(CleanScanResult {
        category: "temp".to_string(),
        files_count: temp_files,
        space_to_free: temp_space,
        description: "System temp directories".to_string(),
        icon: "🗑️".to_string(),
    });
    let _ = window.emit("clean-scan-progress", serde_json::json!({
        "category": "temp",
        "status": "done",
        "files": temp_files,
        "space": temp_space
    }));

    // Scan Browser Cache
    let _ = window.emit("clean-scan-progress", serde_json::json!({
        "category": "browser",
        "status": "scanning"
    }));
    let (browser_files, browser_space, browser_file_list) = scan_browser_cache_with_details();
    total_files += browser_files;
    total_space += browser_space;
    all_files.extend(browser_file_list);
    categories.push(CleanScanResult {
        category: "browser".to_string(),
        files_count: browser_files,
        space_to_free: browser_space,
        description: "Chrome, Firefox, Safari".to_string(),
        icon: "🌐".to_string(),
    });
    let _ = window.emit("clean-scan-progress", serde_json::json!({
        "category": "browser",
        "status": "done",
        "files": browser_files,
        "space": browser_space
    }));

    // Scan Log Files
    let _ = window.emit("clean-scan-progress", serde_json::json!({
        "category": "logs",
        "status": "scanning"
    }));
    let (log_files, log_space, log_file_list) = scan_log_files_with_details();
    total_files += log_files;
    total_space += log_space;
    all_files.extend(log_file_list);
    categories.push(CleanScanResult {
        category: "logs".to_string(),
        files_count: log_files,
        space_to_free: log_space,
        description: "Old system logs".to_string(),
        icon: "📝".to_string(),
    });
    let _ = window.emit("clean-scan-progress", serde_json::json!({
        "category": "logs",
        "status": "done",
        "files": log_files,
        "space": log_space
    }));

    // Scan App Cache
    let _ = window.emit("clean-scan-progress", serde_json::json!({
        "category": "cache",
        "status": "scanning"
    }));
    let (cache_files, cache_space, cache_file_list) = scan_app_cache_with_details();
    total_files += cache_files;
    total_space += cache_space;
    all_files.extend(cache_file_list);
    categories.push(CleanScanResult {
        category: "cache".to_string(),
        files_count: cache_files,
        space_to_free: cache_space,
        description: "Application caches".to_string(),
        icon: "💾".to_string(),
    });
    let _ = window.emit("clean-scan-progress", serde_json::json!({
        "category": "cache",
        "status": "done",
        "files": cache_files,
        "space": cache_space
    }));

    println!("📊 Scan complete: {} files, {} bytes to free", total_files, total_space);

    Ok(CleanScanResults {
        total_files,
        total_space,
        categories,
        files: all_files,
    })
}


// Fonction récursive pour scanner avec détails des fichiers
fn scan_directory_with_details(path: &PathBuf, safe_check: bool, category: &str) -> (u32, u64, Vec<CleanableFile>) {
    let mut files_count = 0u32;
    let mut space = 0u64;
    let mut files = Vec::new();

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            if let Ok(metadata) = entry.metadata() {
                let entry_path = entry.path();
                
                if metadata.is_dir() {
                    // Récursivement scanner les sous-dossiers
                    let (sub_files, sub_space, mut sub_files_list) = scan_directory_with_details(&entry_path, safe_check, category);
                    files_count += sub_files;
                    space += sub_space;
                    files.append(&mut sub_files_list);
                } else if !safe_check || is_safe_to_delete(&entry_path) {
                    // Compter les fichiers et ajouter les détails
                    let size = metadata.len();
                    space += size;
                    files_count += 1;
                    
                    if let Ok(modified) = metadata.modified() {
                        let modified_time = modified.into();
                        files.push(CleanableFile {
                            path: entry_path.to_string_lossy().to_string(),
                            size,
                            category: category.to_string(),
                            modified: modified_time,
                        });
                    }
                }
            }
        }
    }
    
    (files_count, space, files)
}

// Fonctions de scan pour chaque catégorie
fn scan_temp_files_with_details() -> (u32, u64, Vec<CleanableFile>) {
    let mut files_count = 0u32;
    let mut space = 0u64;
    let mut all_files = Vec::new();
    let temp_dirs = get_safe_temp_directories();

    for temp_dir in temp_dirs {
        if temp_dir.exists() {
            let (count, size, mut file_list) = scan_directory_with_details(&temp_dir, true, "temp");
            files_count += count;
            space += size;
            all_files.append(&mut file_list);
        }
    }
    (files_count, space, all_files)
}

fn scan_browser_cache_with_details() -> (u32, u64, Vec<CleanableFile>) {
    let mut files_count = 0u32;
    let mut space = 0u64;
    let mut all_files = Vec::new();
    let cache_dirs = get_browser_cache_directories();

    for cache_dir in cache_dirs {
        if cache_dir.exists() {
            let (count, size, mut file_list) = scan_directory_with_details(&cache_dir, false, "browser");
            files_count += count;
            space += size;
            all_files.append(&mut file_list);
        }
    }
    (files_count, space, all_files)
}

fn scan_log_files_with_details() -> (u32, u64, Vec<CleanableFile>) {
    let mut files_count = 0u32;
    let mut space = 0u64;
    let mut all_files = Vec::new();
    let log_dirs = get_log_directories();

    for log_dir in log_dirs {
        if log_dir.exists() {
            scan_log_files_recursive_with_details(&log_dir, &mut files_count, &mut space, &mut all_files);
        }
    }
    (files_count, space, all_files)
}

fn scan_app_cache_with_details() -> (u32, u64, Vec<CleanableFile>) {
    let mut files_count = 0u32;
    let mut space = 0u64;
    let mut all_files = Vec::new();
    let app_cache_dirs = get_app_cache_directories();

    for cache_dir in app_cache_dirs {
        if cache_dir.exists() {
            let (count, size, mut file_list) = scan_directory_with_details(&cache_dir, true, "cache");
            files_count += count;
            space += size;
            all_files.append(&mut file_list);
        }
    }
    (files_count, space, all_files)
}

fn scan_log_files_recursive_with_details(path: &PathBuf, files_count: &mut u32, space: &mut u64, all_files: &mut Vec<CleanableFile>) {
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            if let Ok(metadata) = entry.metadata() {
                let entry_path = entry.path();
                
                if metadata.is_dir() {
                    // Récursivement scanner les sous-dossiers
                    scan_log_files_recursive_with_details(&entry_path, files_count, space, all_files);
                } else {
                    let path_str = entry_path.to_string_lossy();
                    if path_str.ends_with(".log") {
                        let file_name = entry_path.file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("");
                        if !file_name.contains("system") && !file_name.contains("kernel") {
                            let size_val = metadata.len();
                            *space += size_val;
                            *files_count += 1;
                            
                            if let Ok(modified) = metadata.modified() {
                                let modified_time = modified.into();
                                all_files.push(CleanableFile {
                                    path: entry_path.to_string_lossy().to_string(),
                                    size: size_val,
                                    category: "logs".to_string(),
                                    modified: modified_time,
                                });
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Nettoie les fichiers temporaires système (sécurisé)
pub async fn clean_temp_files() -> Result<CleanResult, String> {
    let mut files_deleted = 0u32;
    let mut space_freed = 0u64;

    // Répertoires temporaires à nettoyer (seulement les sûrs)
    let temp_dirs = get_safe_temp_directories();
    
    println!("🧹 Cleaning temp files from {} directories (RECURSIVE)", temp_dirs.len());

    for temp_dir in temp_dirs {
        println!("  Cleaning: {}", temp_dir.display());
        if temp_dir.exists() {
            clean_directory_recursive(&temp_dir, &mut files_deleted, &mut space_freed);
        }
    }

    println!("✅ Temp files cleaned: {} files, {} bytes freed", files_deleted, space_freed);
    
    Ok(CleanResult {
        id: Uuid::new_v4().to_string(),
        operation: "clean_temp_files".to_string(),
        files_deleted,
        space_freed,
        timestamp: Utc::now(),
        status: "completed".to_string(),
    })
}

/// Nettoie les caches navigateur (sécurisé)
pub async fn clean_browser_cache() -> Result<CleanResult, String> {
    let mut files_deleted = 0u32;
    let mut space_freed = 0u64;

    let cache_dirs = get_browser_cache_directories();
    
    println!("🌐 Cleaning browser cache from {} directories", cache_dirs.len());

    for cache_dir in cache_dirs {
        if cache_dir.exists() {
            println!("  Cleaning: {}", cache_dir.display());
            // Nettoyer récursivement le répertoire de cache
            clean_directory_recursive(&cache_dir, &mut files_deleted, &mut space_freed);
        }
    }

    println!("✅ Browser cache cleaned: {} files, {} bytes freed", files_deleted, space_freed);
    
    Ok(CleanResult {
        id: Uuid::new_v4().to_string(),
        operation: "clean_browser_cache".to_string(),
        files_deleted,
        space_freed,
        timestamp: Utc::now(),
        status: "completed".to_string(),
    })
}

// Fonction récursive pour nettoyer les répertoires
fn clean_directory_recursive(path: &PathBuf, files_deleted: &mut u32, space_freed: &mut u64) {
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            if let Ok(metadata) = entry.metadata() {
                let entry_path = entry.path();
                let size = metadata.len();
                
                if metadata.is_dir() {
                    // Récursivement nettoyer les sous-dossiers
                    clean_directory_recursive(&entry_path, files_deleted, space_freed);
                    // Essayer de supprimer le dossier vide
                    let _ = fs::remove_dir(&entry_path);
                } else {
                    // Supprimer le fichier
                    if fs::remove_file(&entry_path).is_ok() {
                        *space_freed += size;
                        *files_deleted += 1;
                    }
                }
            }
        }
    }
}

fn clean_log_files_recursive(path: &PathBuf, files_deleted: &mut u32, space_freed: &mut u64) {
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            if let Ok(metadata) = entry.metadata() {
                let entry_path = entry.path();
                
                if metadata.is_dir() {
                    // Récursivement nettoyer les sous-dossiers
                    clean_log_files_recursive(&entry_path, files_deleted, space_freed);
                } else {
                    let path_str = entry_path.to_string_lossy();
                    if path_str.ends_with(".log") {
                        let file_name = entry_path.file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("");
                        if !file_name.contains("system") && !file_name.contains("kernel") {
                            let size = metadata.len();
                            if fs::remove_file(&entry_path).is_ok() {
                                *space_freed += size;
                                *files_deleted += 1;
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Nettoie les fichiers de log (sécurisé - seulement logs utilisateur)
pub async fn clean_log_files() -> Result<CleanResult, String> {
    let mut files_deleted = 0u32;
    let mut space_freed = 0u64;

    let log_dirs = get_log_directories();

    println!("📝 Cleaning log files from {} directories (RECURSIVE)", log_dirs.len());

    for log_dir in log_dirs {
        if log_dir.exists() {
            println!("  Cleaning: {}", log_dir.display());
            clean_log_files_recursive(&log_dir, &mut files_deleted, &mut space_freed);
        }
    }

    // Aussi nettoyer les logs dans les caches d'applications
    let app_cache_dirs = get_app_cache_directories();
    for cache_dir in app_cache_dirs {
        if cache_dir.exists() {
            clean_log_files_recursive(&cache_dir, &mut files_deleted, &mut space_freed);
        }
    }

    println!("✅ Log files cleaned: {} files, {} bytes freed", files_deleted, space_freed);

    Ok(CleanResult {
        id: Uuid::new_v4().to_string(),
        operation: "clean_log_files".to_string(),
        files_deleted,
        space_freed,
        timestamp: Utc::now(),
        status: "completed".to_string(),
    })
}

/// Nettoie les caches d'applications (sécurisé)
pub async fn clean_app_cache() -> Result<CleanResult, String> {
    let mut files_deleted = 0u32;
    let mut space_freed = 0u64;

    let app_cache_dirs = get_app_cache_directories();

    println!("💾 Cleaning app cache from {} directories (RECURSIVE)", app_cache_dirs.len());

    for cache_dir in app_cache_dirs {
        println!("  Cleaning: {}", cache_dir.display());
        if cache_dir.exists() {
            clean_directory_recursive(&cache_dir, &mut files_deleted, &mut space_freed);
        }
    }

    Ok(CleanResult {
        id: Uuid::new_v4().to_string(),
        operation: "clean_app_cache".to_string(),
        files_deleted,
        space_freed,
        timestamp: Utc::now(),
        status: "completed".to_string(),
    })
}

// Fonctions utilitaires pour obtenir les répertoires

// Répertoires temporaires sûrs (pas de suppression système critique)
fn get_safe_temp_directories() -> Vec<PathBuf> {
    let mut dirs = Vec::new();

    #[cfg(target_os = "macos")]
    {
        if let Ok(home) = std::env::var("HOME") {
            // Caches utilisateur sûrs à nettoyer
            dirs.push(PathBuf::from(format!("{}/Library/Caches", home)));
            dirs.push(PathBuf::from(format!("{}/Library/Saved Application State", home)));
        }
    }

    #[cfg(target_os = "windows")]
    {
        if let Ok(temp) = std::env::var("TEMP") {
            dirs.push(PathBuf::from(temp));
        }
    }

    dirs
}

// Vérifie si un fichier est sûr à supprimer
fn is_safe_to_delete(path: &PathBuf) -> bool {
    let path_str = path.to_string_lossy().to_lowercase();
    let file_name = path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("");
    
    // Ne JAMAIS supprimer les fichiers système importants
    let unsafe_patterns = [
        ".app/", 
        "system", 
        "kernel",
        "library/preferences",
        "library/application support",
        ".framework",
        "bin/",
        "sbin/",
        "usr/bin",
        "usr/sbin",
    ];
    let has_unsafe_pattern = unsafe_patterns.iter()
        .any(|pattern| path_str.contains(pattern));
    
    if has_unsafe_pattern {
        return false
    }
    
    // Extensions sûres à supprimer
    let safe_extensions = [".tmp", ".temp", ".cache", ".bak", ".old", ".log"];
    let has_safe_ext = safe_extensions.iter()
        .any(|ext| file_name.ends_with(ext));
    
    // Si on est dans un répertoire de cache, c'est généralement sûr
    let is_in_cache = path_str.contains("cache") || 
                      path_str.contains("temp") ||
                      path_str.contains("tmp");
    
    has_safe_ext || is_in_cache
}

fn get_browser_cache_directories() -> Vec<PathBuf> {
    let mut dirs = Vec::new();

    #[cfg(target_os = "macos")]
    {
        if let Ok(home) = std::env::var("HOME") {
            // Chrome cache
            dirs.push(PathBuf::from(format!("{}/Library/Caches/Google/Chrome", home)));
            
            // Firefox cache
            dirs.push(PathBuf::from(format!("{}/Library/Caches/Firefox", home)));
            
            // Safari cache
            dirs.push(PathBuf::from(format!("{}/Library/Caches/com.apple.Safari", home)));
            
            // Brave cache
            dirs.push(PathBuf::from(format!("{}/Library/Caches/com.brave.Browser", home)));
            
            // Edge cache
            dirs.push(PathBuf::from(format!("{}/Library/Caches/Microsoft Edge", home)));
        }
    }

    #[cfg(target_os = "windows")]
    {
        if let Ok(appdata) = std::env::var("APPDATA") {
            dirs.push(PathBuf::from(format!("{}\\Google\\Chrome\\User Data\\Default\\Cache", appdata)));
            dirs.push(PathBuf::from(format!("{}\\Mozilla\\Firefox", appdata)));
            dirs.push(PathBuf::from(format!("{}\\BraveSoftware\\Brave-Browser\\User Data\\Default\\Cache", appdata)));
        }
    }

    dirs
}

fn get_log_directories() -> Vec<PathBuf> {
    let mut dirs = Vec::new();

    #[cfg(target_os = "macos")]
    {
        if let Ok(home) = std::env::var("HOME") {
            // Seulement les logs utilisateur, pas /var/log système
            dirs.push(PathBuf::from(format!("{}/Library/Logs", home)));
        }
    }
    
    #[cfg(target_os = "windows")]
    {
        if let Ok(appdata) = std::env::var("LOCALAPPDATA") {
            dirs.push(PathBuf::from(format!("{}\\Logs", appdata)));
        }
    }

    dirs
}

fn get_app_cache_directories() -> Vec<PathBuf> {
    let mut dirs = Vec::new();

    #[cfg(target_os = "macos")]
    {
        if let Ok(home) = std::env::var("HOME") {
            dirs.push(PathBuf::from(format!("{}/Library/Caches", home)));
            dirs.push(PathBuf::from(format!("{}/Library/Application Support", home)));
        }
    }

    #[cfg(target_os = "windows")]
    {
        if let Ok(appdata) = std::env::var("APPDATA") {
            dirs.push(PathBuf::from(appdata));
        }
        if let Ok(localappdata) = std::env::var("LOCALAPPDATA") {
            dirs.push(PathBuf::from(localappdata));
        }
    }

    dirs
}
