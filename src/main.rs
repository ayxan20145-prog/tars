use clap::{Parser, Subcommand};

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
    Archive { archive: String, files: Vec<String> },
    Extract { archive: String },
}
fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Archive { archive, files } => {
            println!("coming soon");
        }
        Commands::Extract { archive } => {
            println!("coming soon");
        }
    }
}
