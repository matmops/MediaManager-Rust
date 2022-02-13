pub use structopt::StructOpt;

/// Représente les arguments en paramètres de ligne de commande
#[derive(Debug, StructOpt)]
#[structopt(name = "medman", about = "Media manager")]

pub struct CliArguments {
    /// List of available commands in the utility:
    #[structopt(subcommand)]
    pub action: Command,

    /// Chemin où trouver les fichiers à analyser
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

#[derive(Debug, StructOpt)]
pub enum Command {
    /// Search for Media files in directory
    Search,
    /// Scan for Media files in directory
    Scan,
    /// Write into a playlist
    Write2playlist,
}

impl Default for CliArguments {
    fn default() -> Self {
        Self::new()
    }
}

impl CliArguments {
    pub fn new() -> CliArguments {
        CliArguments::from_args()
    }

    pub fn path(&self) -> &std::path::Path {
        self.path.as_path()
    }
}
