use std::fs::File;

use anyhow::Result;
use clap::Parser;
use keepass::{ChallengeResponseKey, Database, DatabaseKey};

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

    /// Do not use a password to decrypt the destination database
    #[clap(long, short)]
    no_password: bool,

    /// Use the same credentials for both databases.
    #[clap(long, short)]
    same_credentials: bool,

    /// Do not save the resulting database.
    #[clap(long, short)]
    dry_run: bool,

    /// The slot number of the yubikey to decrypt the destination database
    #[arg(long)]
    slot: Option<String>,

    /// The serial number of the yubikey to decrypt the destination database
    #[arg(long)]
    serial_number: Option<u32>,

    /// The slot number of the yubikey to decrypt the source database
    #[clap(long)]
    slot_from: Option<String>,

    /// The serial number of the yubikey to decrypt the source database
    #[arg(long)]
    serial_number_from: Option<u32>,

    /// Do not use a password to decrypt the source database
    #[clap(long)]
    no_password_from: bool,

    /// Force saving the database even if warnings were generated.
    #[clap(long, short)]
    force: bool,
}

fn main() -> Result<std::process::ExitCode> {
    let args = KeepassMerge::parse();

    let destination_db_path = args.destination_db;
    let source_db_path = args.source_db;

    let mut destination_db_file = File::open(&destination_db_path)?;
    let mut source_db_file = File::open(&source_db_path)?;

    let mut destination_db_key = DatabaseKey::new();

    if !args.no_password {
        let mut password_prompt = "Password for the destination database: ";
        // Use a slightly more meaningful prompt if the password is that same
        // for both databases.
        if args.same_credentials {
            password_prompt = "Password for the databases: ";
        }

        let destination_db_password =
            rpassword::prompt_password(password_prompt).expect("Could not read password from TTY");
        destination_db_key = destination_db_key.with_password(&destination_db_password);
    }

    // TODO support keyfile

    if let Some(slot) = args.slot {
        let yubikey = ChallengeResponseKey::get_yubikey(args.serial_number)?;
        destination_db_key = destination_db_key
            .with_challenge_response_key(ChallengeResponseKey::YubikeyChallenge(yubikey, slot));
    }

    if destination_db_key.is_empty() {
        return Err(anyhow::format_err!(
            "No database key was provided for destination database."
        ));
    }

    let mut destination_db = Database::open(&mut destination_db_file, destination_db_key.clone())?;

    let source_db = match args.same_credentials {
        true => Database::open(&mut source_db_file, destination_db_key.clone()),
        false => {
            let mut source_db_key = DatabaseKey::new();

            if !args.no_password_from {
                let source_db_password = rpassword::prompt_password("Password for the source database: ")
                    .expect("Could not read password from TTY");

                source_db_key = source_db_key.with_password(&source_db_password);
            }

            // TODO support keyfile

            if let Some(slot) = args.slot_from {
                let yubikey = ChallengeResponseKey::get_yubikey(args.serial_number_from)?;
                source_db_key = source_db_key
                    .with_challenge_response_key(ChallengeResponseKey::YubikeyChallenge(yubikey, slot));
            }

            if source_db_key.is_empty() {
                return Err(anyhow::format_err!(
                    "No database key was provided for source database."
                ));
            }

            Database::open(&mut source_db_file, source_db_key)
        }
    }?;

    let merge_result = match destination_db.merge(&source_db) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("{}", e);
            return Ok(std::process::ExitCode::FAILURE);
        }
    };

    for warning in &merge_result.warnings {
        println!("WARNING: {}", warning);
    }
    if !args.force && !merge_result.warnings.is_empty() {
        println!("Warnings were generated by the merge operation. Not saving the database.");
        return Ok(std::process::ExitCode::FAILURE);
    }

    if merge_result.events.len() == 0 {
        println!("Nothing to merge.");
        return Ok(std::process::ExitCode::SUCCESS);
    }

    for event in merge_result.events {
        println!("{} {:?}", event.node_uuid, event.event_type);
    }
    if args.dry_run {
        println!("Running in dry-run mode. Not saving the database.");
        return Ok(std::process::ExitCode::SUCCESS);
    }

    let mut destination_db_file = File::options().write(true).open(&destination_db_path)?;
    destination_db.save(&mut destination_db_file, destination_db_key)?;
    println!("Databases were merged successfully.");

    Ok(std::process::ExitCode::SUCCESS)
}
