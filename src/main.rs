mod cli;
mod container;
mod cgroups;
mod ls;

use cli::{Cli, Commands};
use clap::Parser;
use container::{delete_container, run_container, delete_all_containers};
use ls::ls;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Hello => {
            println!("Hello la team Padocker 🐳");
        }
        Commands::Run(run) => {
            let container_name = run.name.clone().unwrap_or_else(|| run.program.clone());
            println!("Lancement de {} dans un container", container_name);
            println!("Isolation des fichiers activée ? {}", run.isolate_fs);
            run_container(&run.program, &run.args, run.name.clone(), run.isolate_fs, run.memory_limit);
            
        }
        Commands::Delete { name , all} => {
            if *all {
                delete_all_containers();
            } else if let Some(name) = name {
                delete_container(name);
            } else {
                eprintln!("🥺 Veuillez spécifier le nom avec --name ou --all.");
            }
        }
        Commands::Ls => {
            ls();
        }
    }
}