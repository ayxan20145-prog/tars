use clap::{Parser, Subcommand, ValueEnum};
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
        /// compression type
        #[arg(short, long, value_enum, default_value_t = Compression::None)]
        compression: Compression,

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

#[derive(ValueEnum, Debug, Clone)]
enum Compression {
    None,
    Gzip,
    Xz,
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Archive {
            compression,
            archive,
            files,
        } => {
            let tar_file = File::create(archive).unwrap();

            match compression {
                Compression::None => {
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
                Compression::Gzip => {
                    let gzip =
                        flate2::write::GzEncoder::new(tar_file, flate2::Compression::default());

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
                }
                Compression::Xz => {
                    let xz = xz2::write::XzEncoder::new(tar_file, 6);

                    let mut archive = Builder::new(xz);

                    for file in files {
                        let path = Path::new(&file);

                        if path.is_dir() {
                            archive.append_dir_all(&file, &path).unwrap();
                        } else {
                            archive.append_path(file).unwrap()
                        }
                    }

                    archive.finish().unwrap();
                }
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
