use clap::{ Parser, Subcommand, Args };

#[derive(Parser)]
#[command(name = "padocker", about="C'est comme Docker, mais c'est padocker.")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Hello,
    Run(RunCommand),
    Delete {
        #[arg(long, help = "Nom du Container que l'on veut supprimer.")]
        name: Option<String>,
        #[arg(long, help = "Supprimer tous les containers")]
        all: bool,
    },
    Ls,
}
#[derive(Args, Debug)]
pub struct RunCommand {
    #[arg(help = "Programme que l'on veut lancer.")]
    pub program: String,

    #[arg(help = "Arguments du programme.", trailing_var_arg = true)]
    pub args: Vec<String>,

    #[arg(long = "fs", short = 'f', help = "Pour activer l'isolation des fichiers du Container.")]
    pub isolate_fs: bool,

    #[arg(long = "name", short = 'n', help = "Nom que l'on veut donner au container")]
    pub name: Option<String>,

    #[arg(long = "memory_limit", help = "Limite de mémoire que l'on veut donner à son Container en Mio. Par défaut 1 Gio")]
    pub memory_limit: Option<usize>,
}