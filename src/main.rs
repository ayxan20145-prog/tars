use clap::{Parser, Subcommand};
use std::{fs::File, path::Path};
use tar::{Archive, Builder};

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
        /// use gzip compression
        #[arg(long)]
        gzip: bool,

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

    /// list the contents of a tar archive
    List {
        /// the archive file to list
        archive: String,
    },
}
fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Archive {
            gzip,
            archive,
            files,
        } => {
            let tar_file = File::create(archive).unwrap();

            if gzip {
                let gzip = flate2::write::GzEncoder::new(tar_file, flate2::Compression::default());

                let mut archive = Builder::new(gzip);

                for file in files {
                    let path = Path::new(&file);

                    if path.is_dir() {
                        archive.append_dir_all(&file, &path).unwrap();
                    } else {
                        archive.append_path(file).unwrap();
                    }
                }

                archive.finish().unwrap();
            } else {
                let mut archive = Builder::new(tar_file);

                for file in files {
                    let path = Path::new(&file);

                    if path.is_dir() {
                        archive.append_dir_all(&file, &path).unwrap();
                    } else {
                        archive.append_path(file).unwrap();
                    }
                }

                archive.finish().unwrap();
            }
        }
        Commands::Extract { archive } => {
            let tar_file = File::open(archive).unwrap();
            let mut archive = Archive::new(tar_file);

            archive.unpack(".").unwrap();
        }
        Commands::List { archive } => {
            let tar_file = File::open(archive).unwrap();
            let mut archive = Archive::new(tar_file);

            for entry in archive.entries().unwrap() {
                println!("{}", entry.unwrap().path().unwrap().display());
            }
        }
    }
}
