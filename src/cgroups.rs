use std::fs;


pub fn setup_memory_limit(cgroup_name: &str, memory_limit_in_bytes: usize, pid: i32) {
    let base = "/sys/fs/cgroup";
    let cgroup_path = format!("{}/{}", base, cgroup_name);

    fs::create_dir_all(&cgroup_path).expect("🤕 Impossible de créer le dossier cgroup");

    fs::write(format!("{}/memory.max", cgroup_path), memory_limit_in_bytes.to_string())
        .expect("🪦 Impossible de fixer la limite mémoire");

    fs::write(format!("{}/cgroups.procs", cgroup_path), pid.to_string())
        .expect("🪦 Impossible d'ajouter le processus au cgroup");

    println!("⚙️ Cgroup '{}' configuré avec {} bytes pour PID {}", cgroup_name, memory_limit_in_bytes, pid);
}