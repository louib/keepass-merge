use anyhow::Result;
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

fn main() -> Result<std::process::ExitCode> {
    let args = KeepassMerge::parse();

    let destination_db_path = args.destination_db;
    let source_db_path = args.source_db;

    let mut database_file = std::fs::File::open(&destination_db_path)?;

    let password = rpassword::prompt_password("Password (or blank for none): ")
        .expect("Could not read password from TTY");

    // TODO support keyfile
    // TODO support yubikey
    //
    let mut db = Database::open(&mut database_file, DatabaseKey::with_password(&password))?;
    println!("Enter '?' to print the list of available commands.");

    println!("Hello, world!");

    Ok(std::process::ExitCode::SUCCESS)
}
