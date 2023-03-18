use std::fs::File;

use anyhow::Result;
use clap::{arg, Command, Parser};
use keepass::{Database, DatabaseKey};

/// Contact manager based on the KDBX4 encrypted database format
#[derive(Parser)]
#[clap(name = "keep-in-touch")]
#[clap(version = env!("CARGO_PKG_VERSION"))]
#[clap(about = "CLI tool to merge KDBX (keepass) databases", long_about = None)]
struct KeepassMerge {
    /// The path of the database file to merge to.
    destination_db: String,

    /// The path of the database file to merge from.
    source_db: String,

    /// Disables the password prompt on stdout.
    #[clap(long, short)]
    no_prompt: bool,

    /// Use the same credentials for both databases.
    #[clap(long, short)]
    same_credentials: bool,

    /// Do not save the resulting database.
    #[clap(long, short)]
    dry_run: bool,
}

fn main() -> Result<std::process::ExitCode> {
    let args = KeepassMerge::parse();

    let destination_db_path = args.destination_db;
    let source_db_path = args.source_db;

    let mut destination_db_file = File::open(&destination_db_path)?;
    let mut source_db_file = File::open(&source_db_path)?;

    let destination_db_password =
        rpassword::prompt_password("Password for the destination database (or blank for none): ")
            .expect("Could not read password from TTY");

    // TODO support keyfile
    // TODO support yubikey
    //
    let mut destination_db = Database::open(
        &mut destination_db_file,
        DatabaseKey::with_password(&destination_db_password),
    )?;

    let source_db = match args.same_credentials {
        true => Database::open(
            &mut source_db_file,
            DatabaseKey::with_password(&destination_db_password),
        ),
        false => {
            // TODO support keyfile
            // TODO support yubikey
            //
            let source_db_password = rpassword::prompt_password(
                "Password for the source database (or blank for none): ",
            )
            .expect("Could not read password from TTY");

            Database::open(
                &mut source_db_file,
                DatabaseKey::with_password(&source_db_password),
            )
        }
    }?;

    if args.dry_run {
        panic!("dry-run option is not implemented yet.")
    }

    destination_db.root.merge(&source_db.root);

    let mut destination_db_file = File::options().write(true).open(&destination_db_path)?;
    destination_db.save(
        &mut destination_db_file,
        DatabaseKey::with_password(&destination_db_password),
    )?;
    print!("Databases were merged successfully.");

    Ok(std::process::ExitCode::SUCCESS)
}
