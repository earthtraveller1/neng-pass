use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

use clap::{arg, Command};
use directories::ProjectDirs;
use neng_pass::crypto;

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

fn query_master_key(p_message: &str, p_master_key_file: &mut File) -> Option<String> {
    eprint!("{}", p_message);

    let mut user_input_key = String::new();
    std::io::stdin()
        .read_line(&mut user_input_key)
        .expect("Failed to read the input string.");
    let user_input_key = user_input_key.trim();

    let user_input_key_hashed = crypto::hash(user_input_key);
    let mut actual_key_hashed = Vec::new();
    if p_master_key_file
        .read_to_end(&mut actual_key_hashed)
        .is_err()
    {
        eprintln!(
            "[ERROR]: Failed to read the master key. Maybe you don't have permission to read it?"
        );
        return None;
    }

    if user_input_key_hashed.as_slice() != actual_key_hashed.as_slice() {
        eprintln!("[ERROR]: That is the wrong master key.");
        return None;
    }

    Some(user_input_key.to_string())
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
            let master_key_file = File::open(format!("{}/master_key", data_dir));
            if let Ok(mut master_key_file) = master_key_file {
                if query_master_key("Enter the old master key: ", &mut master_key_file).is_none() {
                    std::process::exit(1);
                }
            }

            let new_key = sub_matches.get_one::<String>("NEW_KEY").unwrap();
            let master_key_file = File::create(format!("{}/master_key", data_dir));

            if let Ok(mut master_key_file) = master_key_file {
                let hashed_new_key = crypto::hash(new_key);
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
