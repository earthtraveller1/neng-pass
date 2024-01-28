use std::{fs::File, io::Write, path::Path};

use clap::{arg, Command};
use directories::ProjectDirs;
use sha2::{Sha512, Digest};

fn cli() -> Command {
    Command::new("neng-pass")
        .about("Basic password manager written in Rust (btw)")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("set-master")
                .about("Sets the master key")
                .arg(arg!(<NEW_KEY> "the master key you want to set it to")),
        )
}

fn main() {
    let project_dirs = ProjectDirs::from("io", "earthtraveller1", "neng-pass");
    let data_dir = match project_dirs.as_ref() {
        Some(project_dirs) => project_dirs.data_dir(),
        None => Path::new("."),
    };

    let data_dir = data_dir.to_str().unwrap();

    let cli_matches = cli().get_matches();
    // Ensure that the data directory exists in the first place.
    if let Err(err) = std::fs::create_dir_all(data_dir) {
        eprintln!("[ERROR]: Failed to create the data directory. {}", err);
        std::process::exit(1);
    }

    eprintln!("[INFO]: Program data are stored in {}", data_dir);

    match cli_matches.subcommand() {
        Some(("set-master", sub_matches)) => {
            let new_key = sub_matches.get_one::<String>("NEW_KEY").unwrap();
            let master_key_file = File::create(format!("{}/master_key", data_dir));

            if let Ok(mut master_key_file) = master_key_file {
                let hashed_new_key = Sha512::digest(new_key);
                if let Err(err) = master_key_file.write_all(hashed_new_key.as_slice()) {
                    eprintln!("[ERROR]: Failed to write to the master key file. {}", err);
                    std::process::exit(1);
                }
            } else {
                eprintln!("[ERROR]: Failed to open or create the master key file.");
                std::process::exit(1);
            }

            eprintln!("Successfully updated the master key file.");
        }
        _ => {
            panic!("truly a bruh moment, this should be unreachable");
        }
    }
}
