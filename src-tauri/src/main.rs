#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

mod commands;
mod models;

use commands::{
    analyzer::{analyze_disk, find_large_files, find_large_files_with_progress, delete_files},
    cleaner::{clean_app_cache, clean_browser_cache, clean_log_files, clean_temp_files, scan_for_cleaning_with_progress},
    optimizer::{disable_startup_program, get_startup_programs, get_system_performance, get_memory_hogs, kill_process},
    malware_detector::{scan_for_malware, scan_for_malware_with_progress, scan_autostart_entries, MalwareFile},
    disk_analysis::{analyze_disk_advanced, DiskAnalysis},
    junk_detector::{scan_for_junk, JunkScanResult},
    virus_scanner::{scan_file_and_report, MalwareScanReport},
};
use models::{CleanResult, DiskStats, LargeFile, StartupProgram, MemoryHog};

// Charger les variables d'environnement depuis .env
fn load_env() {
    dotenv::dotenv().ok();
    println!("✓ Environment variables loaded from .env");
}

#[tauri::command]
async fn analyze_disk_cmd() -> Result<DiskStats, String> {
    analyze_disk().await
}

#[tauri::command]
async fn find_large_files_cmd(limit: u32) -> Result<Vec<LargeFile>, String> {
    find_large_files(limit).await
}

#[tauri::command]
async fn find_large_files_with_progress_cmd(window: tauri::Window, limit: u32) -> Result<Vec<LargeFile>, String> {
    find_large_files_with_progress(window, limit).await
}

#[tauri::command]
async fn delete_files_cmd(paths: Vec<String>) -> Result<serde_json::Value, String> {
    match delete_files(paths).await {
        Ok((deleted, freed)) => Ok(serde_json::json!({
            "deleted": deleted,
            "freed": freed
        })),
        Err(e) => Err(e),
    }
}

#[tauri::command]
async fn get_home_dir_cmd() -> Result<String, String> {
    dirs::home_dir()
        .ok_or("Cannot get home directory".to_string())
        .map(|p| p.display().to_string())
}

#[tauri::command]
async fn scan_for_malware_cmd(root_path: String, limit: u32) -> Result<Vec<MalwareFile>, String> {
    scan_for_malware(&root_path, limit).await
}

#[tauri::command]
async fn scan_for_malware_with_progress_cmd(window: tauri::Window, root_path: String, limit: u32) -> Result<Vec<MalwareFile>, String> {
    scan_for_malware_with_progress(window, &root_path, limit).await
}

#[tauri::command]
async fn scan_autostart_entries_cmd() -> Result<Vec<MalwareFile>, String> {
    scan_autostart_entries().await
}

#[tauri::command]
async fn analyze_disk_advanced_cmd(window: tauri::Window, root_path: String) -> Result<DiskAnalysis, String> {
    analyze_disk_advanced(window, &root_path).await
}

#[tauri::command]
async fn scan_for_junk_cmd(window: tauri::Window, root_path: String) -> Result<JunkScanResult, String> {
    scan_for_junk(window, &root_path).await
}

#[tauri::command]
async fn scan_for_cleaning_cmd(window: tauri::Window) -> Result<crate::models::CleanScanResults, String> {
    scan_for_cleaning_with_progress(window).await
}

#[tauri::command]
async fn clean_temp_files_cmd() -> Result<CleanResult, String> {
    clean_temp_files().await
}

#[tauri::command]
async fn clean_browser_cache_cmd() -> Result<CleanResult, String> {
    clean_browser_cache().await
}

#[tauri::command]
async fn clean_log_files_cmd() -> Result<CleanResult, String> {
    clean_log_files().await
}

#[tauri::command]
async fn clean_app_cache_cmd() -> Result<CleanResult, String> {
    clean_app_cache().await
}

#[tauri::command]
async fn get_startup_programs_cmd() -> Result<Vec<StartupProgram>, String> {
    get_startup_programs().await
}

#[tauri::command]
async fn get_memory_hogs_cmd() -> Result<Vec<MemoryHog>, String> {
    get_memory_hogs().await
}

#[tauri::command]
async fn kill_process_cmd(pid: u32) -> Result<bool, String> {
    kill_process(pid).await
}

#[tauri::command]
async fn disable_startup_program_cmd(name: String) -> Result<bool, String> {
    disable_startup_program(name).await
}

#[tauri::command]
async fn get_system_performance_cmd() -> Result<serde_json::Value, String> {
    match get_system_performance().await {
        Ok(perf) => Ok(serde_json::json!({
            "cpu_usage": perf.cpu_usage,
            "memory_usage": perf.memory_usage,
            "disk_usage": perf.disk_usage,
        })),
        Err(e) => Err(e),
    }
}

#[tauri::command]
async fn scan_file_with_cloudmersive_cmd(file_path: String) -> Result<MalwareScanReport, String> {
    scan_file_and_report(&file_path).await
}

fn main() {
    // Charger les variables d'environnement
    load_env();
    
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            analyze_disk_cmd,
            find_large_files_cmd,
            find_large_files_with_progress_cmd,
            delete_files_cmd,
            get_home_dir_cmd,
            scan_for_malware_cmd,
            scan_for_malware_with_progress_cmd,
            scan_autostart_entries_cmd,
            analyze_disk_advanced_cmd,
            scan_for_junk_cmd,
            scan_for_cleaning_cmd,
            clean_temp_files_cmd,
            clean_browser_cache_cmd,
            clean_log_files_cmd,
            clean_app_cache_cmd,
            get_startup_programs_cmd,
            get_memory_hogs_cmd,
            kill_process_cmd,
            disable_startup_program_cmd,
            get_system_performance_cmd,
            scan_file_with_cloudmersive_cmd,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
