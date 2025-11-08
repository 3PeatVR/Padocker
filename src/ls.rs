use std::fs;
use std::path::Path;
use std::io::{Read};

pub fn ls() {
    let base_path = Path::new("containers");
    
    if !base_path.exists() {
        println!("🙈 Le dossier 'containers' n'existe pas.");
        return;
    }

    println!("{:<20} | {:<10} | {:<15} |", "Nom", "PID", "Limite mémoire");
    println!("{}", "-".repeat(53));

    let entries = fs::read_dir(base_path).expect("Impossible de lire 'containers'");
    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_dir() {
                let name = entry.file_name().into_string().unwrap_or_default();

                let mut pid = "N/A".to_string();
                let mut memory_limit = "N/A".to_string();                
                let cgroup_path = path.join("rootfs").join("sys").join("fs").join("cgroup").join(&name);
                let procs_path = cgroup_path.join("cgroups.procs");

                if let Ok(mut file) = fs::File::open(&procs_path) {
                    let mut contents = String::new();
                    if file.read_to_string(&mut contents).is_ok() {
                        pid = contents.trim().split('\n').next().unwrap_or("N/A").to_string();
                    }
                }

                let mem_path = cgroup_path.join("memory.max");
                if let Ok(mut file) = fs::File::open(&mem_path) {
                    let mut contents = String::new();
                    if file.read_to_string(&mut contents).is_ok() {
                        let trimmed = contents.trim();
                        if let Ok(bytes) = trimmed.parse::<u64>() {
                            let mio = bytes as f64 / (1024.0 * 1024.0);
                            memory_limit = format!("{:.1} Mio", mio);
                        } else {
                            memory_limit = format!("{} o", trimmed);
                        }
                    }
                }
                println!("{:<20} | {:<10} | {:<15} |", name, pid, memory_limit);
                println!("{}", "-".repeat(53));
            }
            
        }
    }
}