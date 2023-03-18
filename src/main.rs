use clap::{arg, Command, Parser};
use keepass::{
    db::{Entry, Node, Value},
    Database, DatabaseKey,
};

/// Contact manager based on the KDBX4 encrypted database format
#[derive(Parser)]
#[clap(name = "keep-in-touch")]
#[clap(version = env!("CARGO_PKG_VERSION"))]
#[clap(about = "Contact manager based on the KDBX4 encrypted database format", long_about = None)]
struct KeepassMerge {
    /// The path of the database file to merge to.
    destination_db: String,
    /// The path of the database file to merge from.
    source_db: String,
    /// Disables the password prompt on stdout.
    #[clap(long, short)]
    no_prompt: bool,
}

fn main() {
    let args = KeepassMerge::parse();

    let database_path = args.path;

    println!("Hello, world!");
}
