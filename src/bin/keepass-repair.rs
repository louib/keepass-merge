/// utility to dump keepass database internal XML data.
use std::fs::File;

use anyhow::Result;
use clap::Parser;
use keepass::{db::NodeRef, Database, DatabaseKey};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Provide a .kdbx database
    in_kdbx: String,

    /// Provide a keyfile
    #[arg(short = 'k', long)]
    keyfile: Option<String>,
}

fn main() -> Result<std::process::ExitCode> {
    let args = Args::parse();

    let mut source = File::open(args.in_kdbx.clone())?;
    let mut key = DatabaseKey::new();

    if let Some(f) = args.keyfile {
        key = key.with_keyfile(&mut File::open(f)?)?;
    }

    let password = rpassword::prompt_password("Password (or blank for none): ")
        .expect("Could not read password from TTY");

    key = key.with_password(&password);

    let mut db = Database::open(&mut source, key)?;

    repair_group(&mut db.root);

    let mut db_file = File::options().write(true).open(args.in_kdbx)?;
    db.save(&mut db_file, DatabaseKey::new().with_password(&password))?;
    println!("Database was repaired successfully.");

    Ok(std::process::ExitCode::SUCCESS)
}

fn repair_group(group: &mut keepass::db::Group) {
    if group.times.get_last_modification().is_none() {
        println!(
            "Group {} did not have a last modification timestamp. Adding one.",
            group.uuid
        );
        group.times.set_last_modification(keepass::db::Times::now());
    }
    if group.times.get_location_changed().is_none() {
        println!(
            "Group {} did not have a location changed timestamp. Adding one.",
            group.uuid
        );
        group.times.set_location_changed(keepass::db::Times::now());
    }

    for child in &mut group.children {
        match child {
            keepass::db::Node::Group(ref mut g) => repair_group(g),
            keepass::db::Node::Entry(ref mut e) => repair_entry(e),
        }
    }
}

fn repair_entry(entry: &mut keepass::db::Entry) {}
