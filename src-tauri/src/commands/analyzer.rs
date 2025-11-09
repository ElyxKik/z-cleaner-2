use crate::models::{DiskStats, LargeFile};
use chrono::Utc;
use std::path::Path;
use walkdir::WalkDir;
use tauri::Window;

const LARGE_FILE_THRESHOLD: u64 = 100_000_000; // 100 MB

/// Analyse l'espace disque et retourne les statistiques
pub async fn analyze_disk() -> Result<DiskStats, String> {
    let home_dir = dirs::home_dir().ok_or("Impossible de trouver le répertoire home")?;
    
    let mut _total_size = 0u64;
    let mut large_files_count = 0u32;
    let mut large_files_size = 0u64;

    for entry in WalkDir::new(&home_dir)
        .max_depth(4)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if let Ok(metadata) = entry.metadata() {
            if metadata.is_file() {
                let size = metadata.len();
                _total_size += size;

                if size > LARGE_FILE_THRESHOLD {
                    large_files_count += 1;
                    large_files_size += size;
                }
            }
        }
    }

    // Obtenir l'espace libre du système
    #[cfg(target_os = "macos")]
    let (free_size, total_capacity) = get_disk_info_macos(&home_dir)?;

    #[cfg(target_os = "windows")]
    let (free_size, total_capacity) = get_disk_info_windows(&home_dir)?;

    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    let (free_size, total_capacity) = (0, _total_size);

    let used_size = total_capacity.saturating_sub(free_size);
    let percentage_used = if total_capacity > 0 {
        (used_size as f64 / total_capacity as f64) * 100.0
    } else {
        0.0
    };

    Ok(DiskStats {
        total_size: total_capacity,
        used_size,
        free_size,
        percentage_used,
        large_files_count,
        large_files_size,
    })
}

/// Trouve les fichiers volumineux avec progression
pub async fn find_large_files_with_progress(window: Window, limit: u32) -> Result<Vec<LargeFile>, String> {
    let home_dir = dirs::home_dir().ok_or("Impossible de trouver le répertoire home")?;
    let mut large_files = Vec::new();
    let mut file_count = 0u32;

    for entry in WalkDir::new(&home_dir)
        .max_depth(4)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        file_count += 1;
        
        // Envoyer la progression tous les 50 fichiers pour une meilleure réactivité
        if file_count % 50 == 0 {
            let _ = window.emit("scan-progress", serde_json::json!({
                "filesScanned": file_count,
                "filesFound": large_files.len(),
            }));
        }

        if let Ok(metadata) = entry.metadata() {
            if metadata.is_file() && metadata.len() > LARGE_FILE_THRESHOLD {
                if let Ok(modified) = metadata.modified() {
                    let modified_time = modified
                        .duration_since(std::time::UNIX_EPOCH)
                        .map(|d| Utc::now() - chrono::Duration::seconds(d.as_secs() as i64))
                        .unwrap_or_else(|_| Utc::now());

                    large_files.push(LargeFile {
                        path: entry.path().display().to_string(),
                        size: metadata.len(),
                        size_mb: metadata.len() as f64 / (1024.0 * 1024.0),
                        modified: modified_time,
                    });
                }
            }
        }

        if large_files.len() >= limit as usize {
            break;
        }
    }

    large_files.sort_by(|a, b| b.size.cmp(&a.size));
    Ok(large_files.into_iter().take(limit as usize).collect())
}

/// Trouve les fichiers volumineux
pub async fn find_large_files(limit: u32) -> Result<Vec<LargeFile>, String> {
    let home_dir = dirs::home_dir().ok_or("Impossible de trouver le répertoire home")?;
    let mut large_files = Vec::new();

    for entry in WalkDir::new(&home_dir)
        .max_depth(4)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if let Ok(metadata) = entry.metadata() {
            if metadata.is_file() && metadata.len() > LARGE_FILE_THRESHOLD {
                if let Ok(modified) = metadata.modified() {
                    let modified_time = modified
                        .duration_since(std::time::UNIX_EPOCH)
                        .map(|d| Utc::now() - chrono::Duration::seconds(d.as_secs() as i64))
                        .unwrap_or_else(|_| Utc::now());

                    large_files.push(LargeFile {
                        path: entry.path().display().to_string(),
                        size: metadata.len(),
                        size_mb: metadata.len() as f64 / (1024.0 * 1024.0),
                        modified: modified_time,
                    });
                }
            }
        }

        if large_files.len() >= limit as usize {
            break;
        }
    }

    large_files.sort_by(|a, b| b.size.cmp(&a.size));
    Ok(large_files.into_iter().take(limit as usize).collect())
}

#[cfg(target_os = "macos")]
fn get_disk_info_macos(path: &Path) -> Result<(u64, u64), String> {
    use std::ffi::CString;
    use std::os::unix::ffi::OsStrExt;
    
    let path_cstring = CString::new(path.as_os_str().as_bytes())
        .map_err(|_| "Invalid path".to_string())?;
    
    unsafe {
        let mut stat: libc::statvfs = std::mem::zeroed();
        if libc::statvfs(path_cstring.as_ptr(), &mut stat) == 0 {
            let block_size = stat.f_frsize as u64;
            let free_blocks = stat.f_bavail as u64;
            let total_blocks = stat.f_blocks as u64;
            
            let free_size = free_blocks * block_size;
            let total_size = total_blocks * block_size;
            
            Ok((free_size, total_size))
        } else {
            Err("Failed to get disk info".to_string())
        }
    }
}

/// Supprime les fichiers spécifiés
pub async fn delete_files(paths: Vec<String>) -> Result<(u32, u64), String> {
    let mut deleted = 0u32;
    let mut freed = 0u64;

    for path in paths {
        match std::fs::metadata(&path) {
            Ok(metadata) => {
                let size = metadata.len();
                match std::fs::remove_file(&path) {
                    Ok(_) => {
                        deleted += 1;
                        freed += size;
                    }
                    Err(e) => {
                        eprintln!("Erreur lors de la suppression de {}: {}", path, e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Erreur lors de la lecture des métadonnées de {}: {}", path, e);
            }
        }
    }

    Ok((deleted, freed))
}

#[cfg(target_os = "windows")]
fn get_disk_info_windows(path: &Path) -> Result<(u64, u64), String> {
    // Implémentation simplifiée pour Windows
    // En production, utiliser winapi pour obtenir les infos précises
    Ok((0, 0))
}
