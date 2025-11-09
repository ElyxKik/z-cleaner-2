use crate::models::{StartupProgram, SystemPerformance, MemoryHog};
use std::path::PathBuf;
use std::process::Command;
use std::fs;

/// Récupère la liste des programmes au démarrage
pub async fn get_startup_programs() -> Result<Vec<StartupProgram>, String> {
    #[cfg(target_os = "macos")]
    {
        return get_macos_startup_programs();
    }

    #[cfg(target_os = "windows")]
    {
        return get_windows_startup_programs();
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        Ok(Vec::new())
    }
}

/// Désactive un programme au démarrage
pub async fn disable_startup_program(name: String) -> Result<bool, String> {
    #[cfg(target_os = "macos")]
    {
        disable_macos_startup_program(&name)?;
    }

    #[cfg(target_os = "windows")]
    {
        disable_windows_startup_program(&name)?;
    }

    Ok(true)
}

#[cfg(target_os = "macos")]
fn get_macos_startup_programs() -> Result<Vec<StartupProgram>, String> {
    let mut programs = Vec::new();

    if let Ok(home) = std::env::var("HOME") {
        let launch_agents_dir = PathBuf::from(format!("{}/Library/LaunchAgents", home));
        
        if let Ok(entries) = std::fs::read_dir(&launch_agents_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().map_or(false, |ext| ext == "plist") {
                    if let Ok(metadata) = entry.metadata() {
                        programs.push(StartupProgram {
                            name: path.file_name()
                                .and_then(|n| n.to_str())
                                .unwrap_or("Unknown")
                                .to_string(),
                            path: path.display().to_string(),
                            enabled: true,
                            size: metadata.len(),
                        });
                    }
                }
            }
        }
    }

    Ok(programs)
}

#[cfg(target_os = "macos")]
fn disable_macos_startup_program(name: &str) -> Result<(), String> {
    // Désactiver un LaunchAgent sur macOS
    if let Ok(home) = std::env::var("HOME") {
        let launch_agents_dir = PathBuf::from(format!("{}/Library/LaunchAgents", home));
        let plist_path = launch_agents_dir.join(name);
        
        if plist_path.exists() {
            // Utiliser launchctl pour décharger le service
            let _label = name.trim_end_matches(".plist");
            let output = Command::new("launchctl")
                .arg("unload")
                .arg(&plist_path)
                .output();
            
            match output {
                Ok(result) => {
                    if result.status.success() {
                        // Optionnellement, déplacer le fichier vers un dossier "disabled"
                        let disabled_dir = launch_agents_dir.join("Disabled");
                        let _ = fs::create_dir_all(&disabled_dir);
                        let new_path = disabled_dir.join(name);
                        
                        if let Err(e) = fs::rename(&plist_path, &new_path) {
                            return Err(format!("Failed to move plist file: {}", e));
                        }
                        
                        Ok(())
                    } else {
                        Err(format!("Failed to unload service: {}", String::from_utf8_lossy(&result.stderr)))
                    }
                }
                Err(e) => Err(format!("Failed to execute launchctl: {}", e)),
            }
        } else {
            Err(format!("LaunchAgent file not found: {}", name))
        }
    } else {
        Err("Could not determine home directory".to_string())
    }
}

#[cfg(target_os = "windows")]
fn get_windows_startup_programs() -> Result<Vec<StartupProgram>, String> {
    // Implémentation pour Windows (lecture du registre)
    Ok(Vec::new())
}

#[cfg(target_os = "windows")]
fn disable_windows_startup_program(_name: &str) -> Result<(), String> {
    // Implémentation pour Windows
    Ok(())
}

/// Obtient les informations de performance système
pub async fn get_system_performance() -> Result<SystemPerformance, String> {
    #[cfg(target_os = "macos")]
    {
        return get_macos_system_performance();
    }

    #[cfg(target_os = "windows")]
    {
        return get_windows_system_performance();
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        Ok(SystemPerformance {
            cpu_usage: 0.0,
            memory_usage: 0.0,
            disk_usage: 0.0,
        })
    }
}

#[cfg(target_os = "macos")]
fn get_macos_system_performance() -> Result<SystemPerformance, String> {
    // CPU Usage - Parser correctement la sortie de top
    let cpu_output = Command::new("sh")
        .args(&["-c", "top -l 2 -n 0 -F | grep 'CPU usage' | tail -1"])
        .output()
        .map_err(|e| e.to_string())?;
    
    let cpu_str = String::from_utf8_lossy(&cpu_output.stdout);
    
    // Format: "CPU usage: 3.57% user, 2.38% sys, 94.04% idle"
    let cpu_usage = if let Some(line) = cpu_str.lines().next() {
        let mut user_pct = 0.0;
        let mut sys_pct = 0.0;
        
        // Extraire user%
        if let Some(user_part) = line.split("user").next() {
            if let Some(num_str) = user_part.split_whitespace().last() {
                user_pct = num_str.trim_end_matches('%').parse::<f64>().unwrap_or(0.0);
            }
        }
        
        // Extraire sys%
        if let Some(sys_part) = line.split("sys").next() {
            if let Some(parts) = sys_part.split(',').nth(1) {
                if let Some(num_str) = parts.split_whitespace().last() {
                    sys_pct = num_str.trim_end_matches('%').parse::<f64>().unwrap_or(0.0);
                }
            }
        }
        
        user_pct + sys_pct
    } else {
        0.0
    };
    
    println!("DEBUG: CPU Usage from top: {:.1}%", cpu_usage);

    // Memory Usage
    let mem_output = Command::new("vm_stat")
        .output()
        .map_err(|e| e.to_string())?;
    
    let mem_str = String::from_utf8_lossy(&mem_output.stdout);
    let mut pages_active = 0u64;
    let mut pages_wired = 0u64;
    
    for line in mem_str.lines() {
        if line.contains("Pages active:") {
            pages_active = line.split(':').nth(1)
                .and_then(|s| s.trim().trim_end_matches('.').parse().ok())
                .unwrap_or(0);
        } else if line.contains("Pages wired down:") {
            pages_wired = line.split(':').nth(1)
                .and_then(|s| s.trim().trim_end_matches('.').parse().ok())
                .unwrap_or(0);
        }
    }
    
    // Sur macOS, la mémoire "utilisée" est principalement active + wired
    // Les pages inactives peuvent être récupérées rapidement
    let page_size = 4096u64;
    
    // Obtenir la mémoire totale du système
    let mem_total_output = Command::new("sysctl")
        .args(&["-n", "hw.memsize"])
        .output()
        .map_err(|e| e.to_string())?;
    
    let mem_total_str = String::from_utf8_lossy(&mem_total_output.stdout);
    let mem_total_bytes = mem_total_str.trim().parse::<u64>().unwrap_or(0);
    
    // Calculer la mémoire utilisée (active + wired uniquement)
    let used_bytes = (pages_active + pages_wired) * page_size;
    let memory_usage = if mem_total_bytes > 0 {
        (used_bytes as f64 / mem_total_bytes as f64) * 100.0
    } else {
        0.0
    };
    
    println!("DEBUG: Memory - Total: {} GB, Used: {} GB, Usage: {:.1}%", 
        mem_total_bytes / (1024*1024*1024), 
        used_bytes / (1024*1024*1024), 
        memory_usage);

    // Disk Usage
    let disk_output = Command::new("df")
        .args(&["-h", "/"])
        .output()
        .map_err(|e| e.to_string())?;
    
    let disk_str = String::from_utf8_lossy(&disk_output.stdout);
    let disk_usage = disk_str
        .lines()
        .nth(1)
        .and_then(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 5 {
                parts[4].trim_end_matches('%').parse::<f64>().ok()
            } else {
                None
            }
        })
        .unwrap_or(0.0);
    
    println!("DEBUG: Disk Usage: {:.1}%", disk_usage);
    
    let result = SystemPerformance {
        cpu_usage: cpu_usage.min(100.0).max(0.0),
        memory_usage: memory_usage.min(100.0).max(0.0),
        disk_usage: disk_usage.min(100.0).max(0.0),
    };
    
    println!("✓ System Performance - CPU: {:.1}%, Memory: {:.1}%, Disk: {:.1}%", 
        result.cpu_usage, result.memory_usage, result.disk_usage);

    Ok(result)
}

#[cfg(target_os = "windows")]
fn get_windows_system_performance() -> Result<SystemPerformance, String> {
    // Implémentation Windows avec wmic ou PowerShell
    Ok(SystemPerformance {
        cpu_usage: 0.0,
        memory_usage: 0.0,
        disk_usage: 0.0,
    })
}

/// Récupère les applications gourmandes en mémoire
pub async fn get_memory_hogs() -> Result<Vec<MemoryHog>, String> {
    #[cfg(target_os = "macos")]
    {
        return get_macos_memory_hogs();
    }

    #[cfg(target_os = "windows")]
    {
        return get_windows_memory_hogs();
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        Ok(Vec::new())
    }
}

#[cfg(target_os = "macos")]
fn get_macos_memory_hogs() -> Result<Vec<MemoryHog>, String> {
    // Utiliser ps pour obtenir les processus avec la plus haute consommation mémoire
    let output = Command::new("ps")
        .args(&["aux"])
        .output()
        .map_err(|e| e.to_string())?;

    let ps_output = String::from_utf8_lossy(&output.stdout);
    let mut hogs = Vec::new();

    // Obtenir la mémoire totale du système
    let mem_total_output = Command::new("sysctl")
        .args(&["-n", "hw.memsize"])
        .output()
        .map_err(|e| e.to_string())?;

    let mem_total_str = String::from_utf8_lossy(&mem_total_output.stdout);
    let mem_total_mb = mem_total_str.trim().parse::<u64>().unwrap_or(0) / (1024 * 1024);

    for line in ps_output.lines().skip(1) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 11 {
            if let (Ok(pid), Ok(mem_percent_str)) = (parts[1].parse::<u32>(), parts[3].parse::<f64>()) {
                let mem_mb = (mem_total_mb as f64 * mem_percent_str) / 100.0;
                
                // Filtrer les processus avec au moins 10 MB de mémoire
                if mem_mb > 10.0 {
                    let name = parts[10].to_string();
                    hogs.push(MemoryHog {
                        name,
                        pid,
                        memory_mb: mem_mb,
                        memory_percent: mem_percent_str,
                    });
                }
            }
        }
    }

    // Trier par consommation mémoire décroissante et garder les top 10
    hogs.sort_by(|a, b| b.memory_mb.partial_cmp(&a.memory_mb).unwrap());
    hogs.truncate(10);

    Ok(hogs)
}

#[cfg(target_os = "windows")]
fn get_windows_memory_hogs() -> Result<Vec<MemoryHog>, String> {
    // Implémentation Windows avec tasklist
    Ok(Vec::new())
}

/// Arrête un processus par son PID
pub async fn kill_process(pid: u32) -> Result<bool, String> {
    #[cfg(target_os = "macos")]
    {
        return kill_macos_process(pid);
    }

    #[cfg(target_os = "windows")]
    {
        return kill_windows_process(pid);
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        Err("Unsupported platform".to_string())
    }
}

#[cfg(target_os = "macos")]
fn kill_macos_process(pid: u32) -> Result<bool, String> {
    let output = Command::new("kill")
        .arg("-9")
        .arg(pid.to_string())
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        println!("✓ Process {} killed successfully", pid);
        Ok(true)
    } else {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        Err(format!("Failed to kill process {}: {}", pid, error_msg))
    }
}

#[cfg(target_os = "windows")]
fn kill_windows_process(pid: u32) -> Result<bool, String> {
    let output = Command::new("taskkill")
        .args(&["/PID", &pid.to_string(), "/F"])
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok(true)
    } else {
        Err("Failed to kill process".to_string())
    }
}
