use clap::Parser;

#[derive(Parser)]
#[command(
    name = "tars",
    version,
    about = "a simple frontend for working with tar archives without having to remember complicated arguments"
)]
struct Cli {
    command: String,
}
fn main() {
    let args = Cli::parse();
}
