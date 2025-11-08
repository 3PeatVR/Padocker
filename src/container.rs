use crate::cgroups::setup_memory_limit;

use std::fs;
use std::path::Path;
use std::process::Command;
use std::ffi::{CString, CStr};

use nix::sched::{CloneFlags, unshare};
use nix::unistd::{execvp, sethostname, fork, ForkResult, chroot, chdir};
use nix::mount::{mount, MsFlags};

pub fn run_container(program: &str, args: &[String], container_name: Option<String>, fs_isolate: bool, memory_limit: Option<usize>) {
    println!("Préparation du container");

    let rootfs_path = if fs_isolate {
        let path_containers = Path::new("./containers");

        if !path_containers.exists() {
            fs::create_dir_all(&path_containers).expect("Erreur lors de la création du dossier containers");
        } 

        let container_path = format!("containers/{}", container_name.clone().unwrap_or_else(|| program.to_string()));
        let rootfs = format!("{}/rootfs", container_path);
        let rootfs_path = Path::new(&rootfs);

        if !rootfs_path.exists() {
            fs::create_dir_all(&rootfs_path).expect("Erreur lors de la création du dossier rootfs du container");
        

            let status = Command::new("debootstrap").
            args([
                "--variant=minbase",
                "stable",
                &rootfs,
                "http://deb.debian.org/debian",
            ])
            .status()
            .expect("Erreur lors du debootstrap 😢");

            if !status.success() {
                panic!("Échec du debootstrap");
            }
        }

        Some(rootfs)
    } else {
        None
    };

    unshare(
        CloneFlags::CLONE_NEWUTS |
        CloneFlags::CLONE_NEWPID |
        CloneFlags::CLONE_NEWNS
    ).expect("Échec lors de la création du namespace !");

    let hostname = container_name.as_deref().unwrap_or("Padocker");
    sethostname(hostname).expect("Erreur lors du changement d'hostname.");

    if let Some(ref rootfs) = rootfs_path {
        chroot(Path::new(rootfs)).expect("Échec du chroot !");
        chdir("/").expect("Échec du chdir");

        mount(
            Some("proc"),
            "/proc",
            Some("proc"),
            MsFlags::MS_NOSUID | MsFlags::MS_NOEXEC | MsFlags::MS_NODEV,
            None::<&str>,
        ).expect("Montage /proc échoué");
    } else {

        mount(
            Some("proc"),
            "/proc",
            Some("proc"),
            MsFlags::MS_NOSUID | MsFlags::MS_NOEXEC | MsFlags::MS_NODEV,
            None::<&str>,
        ).expect("Échec lors du montage de /proc");
    }
    
    match unsafe { fork() } {
        Ok(ForkResult::Child) => {
            let cmd = CString::new(program).unwrap();
            let cstrings: Vec<CString> = std::iter::once(program.to_string())
                .chain(args.iter().cloned())
                .map(|s| CString::new(s).unwrap())
                .collect();

            let cstr_args: Vec<&CStr> = cstrings.iter().map(|s| s.as_c_str()).collect();

            execvp(&cmd, &cstr_args).expect("Échec de execvp");
        }
        Ok(ForkResult::Parent { child }) => {
            if fs_isolate {
                let limit = memory_limit.unwrap_or(1024) * 1024 * 1024;
                let group_name = container_name.clone().unwrap_or_else(|| "default".to_string());
                setup_memory_limit(&group_name, limit, child.as_raw());
            } else {
                println!("🙅‍♀️ Pas de limitation de mémoire");
            }
            println!("Processus enfant: PID {}", child);
            nix::sys::wait::waitpid(child, None).expect("Erreur waitpid");
        }
        Err(_) => {
            eprintln!("Erreur lors du Fork");
        }
    }
}

pub fn delete_container(name: &str) {
    if name.contains("..") || name.contains('/') {
        eprintln!("😬 Non invalide");
        return;
    }

    let base_path = Path::new("containers").canonicalize().expect("Impossible d'accéder au dossier 'containers'.");
    let container_path = base_path.join(name);
    
    match container_path.canonicalize() {
        Ok(canon_path) => {
            if !canon_path.starts_with(&base_path) {
                eprintln!("🚨 CHEMIN INVALIDE !!! Impossible de supprimer en dehors du dossier 'containers'");
                return;
            }
            if !canon_path.exists() {
                eprint!("💔 Le container '{}' n'existe pas.", name);
                return;
            }
            let proc_path = container_path.join("rootfs/proc");
            if proc_path.exists() {
                let _ = Command::new("sudo")
                    .arg("umount")
                    .arg(proc_path.to_str().unwrap())
                    .status();
            }
            let status = Command::new("sudo")
            .arg("rm")
            .arg("-rf")
            .arg(canon_path.to_str().unwrap())
            .status()
            .expect("Erreur lors de la supression du container.");
        
            if status.success() {
                println!("Container '{}' supprimé avec succès 🐋", name);
            } else {
                eprintln!("Problème lors de la suppression du container.");
            }
        }
        Err(_) => {
            eprintln!("Chemin invalide impossible de résoudre '{}'", name);
        }
    }
}

pub fn delete_all_containers() {
    let base_path = Path::new("containers");

    if !base_path.exists() {
        println!("😿 Le dossier 'containers' n'existe pas.");
        return;
    }

    let canon_base = base_path.canonicalize().expect("Impossible de résoudre le chemin");
    let entries: Vec<_> = std::fs::read_dir(&canon_base)
        .expect("Erreur lors de la lecture du dossier 'containers'")
        .collect();

    if entries.is_empty() {
        println!("🤯 'containers est vide !");
        return;
    }

    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_dir() {
                let path_str = path.to_str().unwrap();
                println!("🐋 Suppression du container '{}'", path_str);

                let proc_path = path.join("rootfs/proc");
                if proc_path.exists() {
                    let status = Command::new("sudo")
                        .arg("umount")
                        .arg("-l") // lazy unmount
                        .arg(proc_path.to_str().unwrap())
                        .status()
                        .expect("Échec du umount");

                    if !status.success() {
                        eprintln!("⚠️ Umount échoué sur '{}'", proc_path.display());
                    }
                }
                let status = Command::new("sudo")
                    .arg("rm")
                    .arg("-rf")
                    .arg(path_str)
                    .status()
                    .expect("Erreur lors de la suppression du container");

                if status.success() {
                    println!("🐳 Container '{}' supprimé", path_str);
                } else {
                    eprintln!("😿 Échec de la suppression du container '{}'", path_str);
                }
            }
        }
    }
}
