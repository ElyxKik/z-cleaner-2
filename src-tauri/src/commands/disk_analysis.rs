use std::collections::HashMap;
use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
use tauri::Window as TauriWindow;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskAnalysis {
    pub total_space: u64,
    pub used_space: u64,
    pub free_space: u64,
    pub usage_percentage: f64,
    pub partitions: Vec<PartitionInfo>,
    pub file_type_distribution: HashMap<String, FileTypeStats>,
    pub largest_folders: Vec<FolderStats>,
    pub duplicate_files: Vec<DuplicateGroup>,
    pub hidden_files_count: u32,
    pub hidden_files_size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartitionInfo {
    pub name: String,
    pub mount_point: String,
    pub total_space: u64,
    pub free_space: u64,
    pub file_system: String,
    pub disk_type: String, // "SSD", "HDD", "USB", "Network"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileTypeStats {
    pub extension: String,
    pub count: u32,
    pub total_size: u64,
    pub percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderStats {
    pub path: String,
    pub size: u64,
    pub file_count: u32,
    pub subfolder_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateGroup {
    pub hash: String,
    pub files: Vec<String>,
    pub size: u64,
    pub wasted_space: u64,
}

pub async fn analyze_disk_advanced(window: TauriWindow, root_path: &str) -> Result<DiskAnalysis, String> {
    let mut file_type_distribution: HashMap<String, FileTypeStats> = HashMap::new();
    let mut largest_folders: Vec<FolderStats> = Vec::new();
    let mut hidden_files_count = 0u32;
    let mut hidden_files_size = 0u64;
    let mut scanned = 0u32;

    // Analyser l'espace disque
    let (total_space, used_space, free_space) = get_disk_space(root_path)?;
    let usage_percentage = (used_space as f64 / total_space as f64) * 100.0;

    // Obtenir les partitions
    let partitions = get_partitions()?;

    // Scanner les fichiers
    scan_directory_for_analysis(
        &window,
        root_path,
        &mut file_type_distribution,
        &mut largest_folders,
        &mut hidden_files_count,
        &mut hidden_files_size,
        &mut scanned,
        0,
    )?;

    // Trier les dossiers par taille
    largest_folders.sort_by(|a, b| b.size.cmp(&a.size));
    largest_folders.truncate(50);

    // Calculer les pourcentages pour les types de fichiers
    let total_files_size: u64 = file_type_distribution.values().map(|s| s.total_size).sum();
    for stats in file_type_distribution.values_mut() {
        stats.percentage = (stats.total_size as f64 / total_files_size as f64) * 100.0;
    }

    Ok(DiskAnalysis {
        total_space,
        used_space,
        free_space,
        usage_percentage,
        partitions,
        file_type_distribution,
        largest_folders,
        duplicate_files: vec![], // TODO: Implémenter la détection de doublons
        hidden_files_count,
        hidden_files_size,
    })
}

fn get_disk_space(path: &str) -> Result<(u64, u64, u64), String> {
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        let output = Command::new("df")
            .arg("-k")
            .arg(path)
            .output()
            .map_err(|e| e.to_string())?;
        
        let output_str = String::from_utf8_lossy(&output.stdout);
        let lines: Vec<&str> = output_str.lines().collect();
        
        if lines.len() >= 2 {
            let parts: Vec<&str> = lines[1].split_whitespace().collect();
            if parts.len() >= 4 {
                let total = parts[1].parse::<u64>().unwrap_or(0) * 1024;
                let used = parts[2].parse::<u64>().unwrap_or(0) * 1024;
                let free = parts[3].parse::<u64>().unwrap_or(0) * 1024;
                return Ok((total, used, free));
            }
        }
    }

    #[cfg(target_os = "windows")]
    {
        use std::os::windows::fs::MetadataExt;
        if let Ok(metadata) = fs::metadata(path) {
            // Windows-specific implementation
            let total = metadata.file_size();
            return Ok((total, 0, 0));
        }
    }

    Ok((0, 0, 0))
}

fn get_partitions() -> Result<Vec<PartitionInfo>, String> {
    let mut partitions = Vec::new();

    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        let output = Command::new("df")
            .arg("-h")
            .output()
            .map_err(|e| e.to_string())?;
        
        let output_str = String::from_utf8_lossy(&output.stdout);
        for line in output_str.lines().skip(1) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 9 {
                partitions.push(PartitionInfo {
                    name: parts[0].to_string(),
                    mount_point: parts[8].to_string(),
                    total_space: parse_size(parts[1]),
                    free_space: parse_size(parts[3]),
                    file_system: "APFS".to_string(),
                    disk_type: detect_disk_type(&parts[0]),
                });
            }
        }
    }

    Ok(partitions)
}

fn parse_size(size_str: &str) -> u64 {
    let size_str = size_str.trim();
    let multiplier = match size_str.chars().last() {
        Some('K') | Some('k') => 1024,
        Some('M') | Some('m') => 1024 * 1024,
        Some('G') | Some('g') => 1024 * 1024 * 1024,
        Some('T') | Some('t') => 1024 * 1024 * 1024 * 1024,
        _ => 1,
    };

    let num_str: String = size_str.chars().filter(|c| c.is_numeric() || *c == '.').collect();
    num_str.parse::<f64>().unwrap_or(0.0) as u64 * multiplier
}

fn detect_disk_type(device: &str) -> String {
    if device.contains("disk") {
        "SSD".to_string()
    } else if device.contains("usb") {
        "USB".to_string()
    } else {
        "HDD".to_string()
    }
}

fn scan_directory_for_analysis(
    window: &TauriWindow,
    path: &str,
    file_type_distribution: &mut HashMap<String, FileTypeStats>,
    largest_folders: &mut Vec<FolderStats>,
    hidden_files_count: &mut u32,
    hidden_files_size: &mut u64,
    scanned: &mut u32,
    depth: u32,
) -> Result<(), String> {
    if depth > 10 {
        return Ok(());
    }

    let mut folder_size = 0u64;
    let mut file_count = 0u32;
    let mut subfolder_count = 0u32;

    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries.flatten() {
                *scanned += 1;

                // Émettre la progression tous les 500 fichiers
                if *scanned % 500 == 0 {
                    let _ = window.emit("disk-analysis-progress", serde_json::json!({
                        "filesScanned": *scanned,
                    }));
                }

                if let Ok(metadata) = entry.metadata() {
                    let path_str = entry.path().display().to_string();
                    let file_name = entry.file_name().to_string_lossy().to_string();

                    // Détecter les fichiers cachés
                    if file_name.starts_with('.') {
                        *hidden_files_count += 1;
                        *hidden_files_size += metadata.len();
                    }

                    if metadata.is_file() {
                        file_count += 1;
                        folder_size += metadata.len();

                        // Analyser l'extension
                        if let Some(ext) = Path::new(&path_str).extension() {
                            let ext_str = ext.to_string_lossy().to_lowercase();
                            file_type_distribution
                                .entry(ext_str.clone())
                                .and_modify(|stats| {
                                    stats.count += 1;
                                    stats.total_size += metadata.len();
                                })
                                .or_insert(FileTypeStats {
                                    extension: ext_str,
                                    count: 1,
                                    total_size: metadata.len(),
                                    percentage: 0.0,
                                });
                        }
                    } else if metadata.is_dir() && *scanned < 50000 {
                        subfolder_count += 1;
                        let _ = scan_directory_for_analysis(
                            window,
                            &path_str,
                            file_type_distribution,
                            largest_folders,
                            hidden_files_count,
                            hidden_files_size,
                            scanned,
                            depth + 1,
                        );
                    }
                }
            }

            // Ajouter ce dossier aux statistiques
            if folder_size > 1024 * 1024 * 10 {
                // Plus de 10MB
                largest_folders.push(FolderStats {
                    path: path.to_string(),
                    size: folder_size,
                    file_count,
                    subfolder_count,
                });
            }
        }
        Err(_) => {}
    }

    Ok(())
}
