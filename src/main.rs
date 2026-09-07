use clap::{Parser, Subcommand};
use std::fs::File;
use tar::Builder;

#[derive(Parser)]
#[command(
    name = "tars",
    version,
    about = "a simple frontend for working with tar archives without having to remember complicated arguments"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    /// create a tar archive
    Archive {
        /// the archive file to create
        archive: String,

        /// files to archive
        files: Vec<String>,
    },

    /// extract a tar archive
    Extract {
        /// the archive file to extract
        archive: String,
    },
}
fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Archive { archive, files } => {
            let tar_file = File::create(archive).unwrap();
            let mut archive = Builder::new(tar_file);

            for file in files {
                archive.append_path(file).unwrap();
            }
        }
        Commands::Extract { archive } => {
            println!("coming soon");
        }
    }
}
