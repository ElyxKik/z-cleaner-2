use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
use tauri::Window as TauriWindow;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JunkFile {
    pub path: String,
    pub size: u64,
    pub category: String, // "cache", "logs", "temp", "trash", "thumbnails", "crash_dumps"
    pub description: String,
    pub safe_to_delete: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JunkScanResult {
    pub total_junk_size: u64,
    pub junk_files: Vec<JunkFile>,
    pub categories: Vec<JunkCategory>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JunkCategory {
    pub name: String,
    pub count: u32,
    pub total_size: u64,
}

// Patterns de fichiers temporaires
const TEMP_EXTENSIONS: &[&str] = &[".tmp", ".temp", ".bak", ".old", ".cache", ".dmp", ".log"];
const CACHE_DIRS: &[&str] = &[
    "Library/Caches",
    "Library/Application Support/Google/Chrome/Default/Cache",
    "Library/Application Support/Firefox/Profiles",
    "Library/Safari/LocalStorage",
    ".cache",
    "AppData/Local/Temp",
    "AppData/Local/Microsoft/Windows/INetCache",
];

const LOG_DIRS: &[&str] = &[
    "Library/Logs",
    "/var/log",
    "AppData/Local/Temp",
];

const THUMBNAIL_FILES: &[&str] = &["thumbs.db", ".DS_Store", "Thumbs.db", "desktop.ini"];

pub async fn scan_for_junk(window: TauriWindow, root_path: &str) -> Result<JunkScanResult, String> {
    let mut junk_files = Vec::new();
    let mut scanned = 0u32;

    // Scanner les répertoires de cache
    for cache_dir in CACHE_DIRS {
        let full_path = format!("{}/{}", root_path, cache_dir);
        scan_directory_for_junk(
            &window,
            &full_path,
            &mut junk_files,
            &mut scanned,
            "cache",
            0,
        )?;
    }

    // Scanner les logs
    for log_dir in LOG_DIRS {
        let full_path = if log_dir.starts_with('/') {
            log_dir.to_string()
        } else {
            format!("{}/{}", root_path, log_dir)
        };
        scan_directory_for_junk(
            &window,
            &full_path,
            &mut junk_files,
            &mut scanned,
            "logs",
            0,
        )?;
    }

    // Scanner les fichiers temporaires dans tout le système
    scan_for_temp_files(&window, root_path, &mut junk_files, &mut scanned)?;

    // Scanner la corbeille
    scan_trash(&window, root_path, &mut junk_files, &mut scanned)?;

    // Calculer les statistiques par catégorie
    let categories = calculate_categories(&junk_files);
    let total_junk_size: u64 = junk_files.iter().map(|f| f.size).sum();

    Ok(JunkScanResult {
        total_junk_size,
        junk_files,
        categories,
    })
}

fn scan_directory_for_junk(
    window: &TauriWindow,
    path: &str,
    junk_files: &mut Vec<JunkFile>,
    scanned: &mut u32,
    category: &str,
    depth: u32,
) -> Result<(), String> {
    if depth > 5 || junk_files.len() > 10000 {
        return Ok(());
    }

    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries.flatten() {
                *scanned += 1;

                // Émettre la progression
                if *scanned % 200 == 0 {
                    let _ = window.emit("junk-scan-progress", serde_json::json!({
                        "filesScanned": *scanned,
                        "junkFound": junk_files.len(),
                    }));
                }

                if let Ok(metadata) = entry.metadata() {
                    let path_str = entry.path().display().to_string();

                    if metadata.is_file() {
                        // Vérifier si c'est un fichier de cache/log
                        if is_junk_file(&path_str, category) {
                            junk_files.push(JunkFile {
                                path: path_str.clone(),
                                size: metadata.len(),
                                category: category.to_string(),
                                description: get_junk_description(category),
                                safe_to_delete: is_safe_to_delete(category),
                            });
                        }
                    } else if metadata.is_dir() && *scanned < 50000 {
                        let _ = scan_directory_for_junk(
                            window,
                            &path_str,
                            junk_files,
                            scanned,
                            category,
                            depth + 1,
                        );
                    }
                }
            }
        }
        Err(_) => {}
    }

    Ok(())
}

fn scan_for_temp_files(
    window: &TauriWindow,
    root_path: &str,
    junk_files: &mut Vec<JunkFile>,
    scanned: &mut u32,
) -> Result<(), String> {
    scan_directory_for_temp(window, root_path, junk_files, scanned, 0)
}

fn scan_directory_for_temp(
    window: &TauriWindow,
    path: &str,
    junk_files: &mut Vec<JunkFile>,
    scanned: &mut u32,
    depth: u32,
) -> Result<(), String> {
    if depth > 8 || junk_files.len() > 10000 {
        return Ok(());
    }

    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries.flatten() {
                *scanned += 1;

                if *scanned % 200 == 0 {
                    let _ = window.emit("junk-scan-progress", serde_json::json!({
                        "filesScanned": *scanned,
                        "junkFound": junk_files.len(),
                    }));
                }

                if let Ok(metadata) = entry.metadata() {
                    let path_str = entry.path().display().to_string();
                    let file_name = entry.file_name().to_string_lossy().to_lowercase();

                    if metadata.is_file() {
                        // Vérifier l'extension
                        if let Some(ext) = Path::new(&path_str).extension() {
                            let ext_str = format!(".{}", ext.to_string_lossy().to_lowercase());
                            if TEMP_EXTENSIONS.contains(&ext_str.as_str()) {
                                junk_files.push(JunkFile {
                                    path: path_str.clone(),
                                    size: metadata.len(),
                                    category: "temp".to_string(),
                                    description: "Temporary file".to_string(),
                                    safe_to_delete: true,
                                });
                            }
                        }

                        // Vérifier les fichiers de miniatures
                        if THUMBNAIL_FILES.contains(&file_name.as_str()) {
                            junk_files.push(JunkFile {
                                path: path_str.clone(),
                                size: metadata.len(),
                                category: "thumbnails".to_string(),
                                description: "Thumbnail cache file".to_string(),
                                safe_to_delete: true,
                            });
                        }
                    } else if metadata.is_dir() && *scanned < 50000 {
                        let _ = scan_directory_for_temp(window, &path_str, junk_files, scanned, depth + 1);
                    }
                }
            }
        }
        Err(_) => {}
    }

    Ok(())
}

fn scan_trash(
    window: &TauriWindow,
    root_path: &str,
    junk_files: &mut Vec<JunkFile>,
    scanned: &mut u32,
) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        let trash_path = format!("{}/.Trash", root_path);
        scan_directory_for_junk(window, &trash_path, junk_files, scanned, "trash", 0)?;
    }

    #[cfg(target_os = "windows")]
    {
        // Windows Recycle Bin
        let trash_path = "C:\\$Recycle.Bin";
        scan_directory_for_junk(window, trash_path, junk_files, scanned, "trash", 0)?;
    }

    Ok(())
}

fn is_junk_file(path: &str, category: &str) -> bool {
    match category {
        "cache" => path.contains("Cache") || path.contains("cache"),
        "logs" => path.ends_with(".log") || path.contains("/Logs/"),
        "temp" => {
            if let Some(ext) = Path::new(path).extension() {
                let ext_str = format!(".{}", ext.to_string_lossy().to_lowercase());
                TEMP_EXTENSIONS.contains(&ext_str.as_str())
            } else {
                false
            }
        }
        _ => false,
    }
}

fn get_junk_description(category: &str) -> String {
    match category {
        "cache" => "Browser or application cache".to_string(),
        "logs" => "System or application log file".to_string(),
        "temp" => "Temporary file".to_string(),
        "trash" => "File in trash/recycle bin".to_string(),
        "thumbnails" => "Thumbnail cache".to_string(),
        "crash_dumps" => "Crash dump or error report".to_string(),
        _ => "Junk file".to_string(),
    }
}

fn is_safe_to_delete(category: &str) -> bool {
    matches!(category, "cache" | "temp" | "thumbnails" | "logs")
}

fn calculate_categories(junk_files: &[JunkFile]) -> Vec<JunkCategory> {
    let mut categories: std::collections::HashMap<String, (u32, u64)> = std::collections::HashMap::new();

    for file in junk_files {
        categories
            .entry(file.category.clone())
            .and_modify(|(count, size)| {
                *count += 1;
                *size += file.size;
            })
            .or_insert((1, file.size));
    }

    categories
        .into_iter()
        .map(|(name, (count, total_size))| JunkCategory {
            name,
            count,
            total_size,
        })
        .collect()
}
