use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Résultat d'une opération de nettoyage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleanResult {
    pub id: String,
    pub operation: String,
    pub files_deleted: u32,
    pub space_freed: u64,
    pub timestamp: DateTime<Utc>,
    pub status: String,
}

/// Informations sur un fichier volumineux
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LargeFile {
    pub path: String,
    pub size: u64,
    pub size_mb: f64,
    pub modified: DateTime<Utc>,
}

/// Statistiques du disque
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskStats {
    pub total_size: u64,
    pub used_size: u64,
    pub free_size: u64,
    pub percentage_used: f64,
    pub large_files_count: u32,
    pub large_files_size: u64,
}

/// Information sur un programme au démarrage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartupProgram {
    pub name: String,
    pub path: String,
    pub enabled: bool,
    pub size: u64,
}

/// Application gourmande en mémoire
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryHog {
    pub name: String,
    pub pid: u32,
    pub memory_mb: f64,
    pub memory_percent: f64,
}

/// Catégorie de nettoyage
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct CleanCategory {
    pub name: String,
    pub description: String,
    pub files_count: u32,
    pub space_to_free: u64,
    pub enabled: bool,
}

/// Configuration de l'application
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct AppConfig {
    pub theme: String,
    pub auto_scan: bool,
    pub scan_interval_hours: u32,
    pub safe_mode: bool,
    pub language: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            theme: "dark".to_string(),
            auto_scan: false,
            scan_interval_hours: 24,
            safe_mode: true,
            language: "en".to_string(),
        }
    }
}

/// Informations de performance système
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemPerformance {
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub disk_usage: f64,
}

/// Résultat du scan d'une catégorie de nettoyage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleanScanResult {
    pub category: String,
    pub files_count: u32,
    pub space_to_free: u64,
    pub description: String,
    pub icon: String,
}

/// Fichier trouvé lors du scan de nettoyage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleanableFile {
    pub path: String,
    pub size: u64,
    pub category: String,
    pub modified: DateTime<Utc>,
}

/// Résultats du scan complet de nettoyage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleanScanResults {
    pub total_files: u32,
    pub total_space: u64,
    pub categories: Vec<CleanScanResult>,
    pub files: Vec<CleanableFile>,
}
