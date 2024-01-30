use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

use aes::cipher::{generic_array::GenericArray, BlockEncrypt, KeyInit};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use clap::{arg, Command};
use directories::ProjectDirs;
use neng_pass::crypto;
use rand::{distributions, Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;

const MAX_MASTER_KEY_LEN: usize = 32;
const MAX_PASSWORD_LEN: usize = 16;

fn cli() -> Command {
    Command::new("neng-pass")
        .about("Basic password manager written in Rust (btw)")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(Command::new("set-master").about("Sets the master key"))
        .subcommand(
            Command::new("new")
                .about("Creates a new password with the specified name.")
                .arg(arg!(<NAME> "The you want to assign to the password.")),
        )
}

fn query_master_key(p_master_key_file: &mut File) -> Option<String> {
    let user_input_key = rpassword::prompt_password("Please enter the master password: ").ok()?;
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

    let actual_key_hashed = String::from_utf8(actual_key_hashed).ok()?;

    let argon2 = Argon2::default();
    let actual_key_hashed = PasswordHash::new(&actual_key_hashed).unwrap();

    if argon2
        .verify_password(user_input_key.as_bytes(), &actual_key_hashed)
        .is_err()
    {
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
        Some(("set-master", _)) => {
            let master_key_file = File::open(format!("{}/master_key", data_dir));
            if master_key_file.is_ok() {
                eprintln!("You have already set the master key! Resetting the master key will break the database! Don't do it!");
                std::process::exit(1);
            }

            let new_key = rpassword::prompt_password("Enter a new master key: ").unwrap();
            let new_key_confirmation =
                rpassword::prompt_password("Confirm your master password: ").unwrap();
            if new_key != new_key_confirmation {
                eprintln!("The passwords you entered do not match!");
                std::process::exit(1);
            }

            if new_key.len() > MAX_MASTER_KEY_LEN {
                eprintln!(
                    "Your password is way too long! Limits to {} only.",
                    MAX_MASTER_KEY_LEN
                );
                std::process::exit(1);
            }

            let master_key_file = File::create(format!("{}/master_key", data_dir));

            if let Ok(mut master_key_file) = master_key_file {
                let hashed_new_key = crypto::hash(&new_key).unwrap();
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
        Some(("new", sub_matches)) => {
            let master_key_file = File::open(format!("{}/master_key", data_dir));
            let mut master_key = match master_key_file {
                Ok(mut master_key_file) => match query_master_key(&mut master_key_file) {
                    Some(master_key) => master_key,
                    None => std::process::exit(1),
                },
                Err(_) => {
                    eprintln!("It appears that you didn't set a master key yet, or I can't access the file for some reasons.");
                    std::process::exit(1);
                }
            };

            while master_key.len() < MAX_MASTER_KEY_LEN {
                master_key.push(' ');
            }

            let rng = ChaCha20Rng::from_entropy();
            let password = rng
                .sample_iter(&distributions::Alphanumeric)
                .take(MAX_PASSWORD_LEN)
                .collect::<Vec<u8>>();

            let mut master_key_block = [b' '; MAX_MASTER_KEY_LEN];
            master_key_block.copy_from_slice(master_key.as_bytes());
            let master_key_block = GenericArray::from(master_key_block);
            let name = sub_matches.get_one::<String>("NAME").unwrap();

            let mut password_block = [0u8; MAX_PASSWORD_LEN];
            password_block.copy_from_slice(&password);
            let mut password_block = GenericArray::from(password_block);

            let cipher = aes::Aes256::new(&master_key_block);
            cipher.encrypt_block(&mut password_block);

            // For debugging purposes
            eprintln!(
                "The generated password is {}",
                String::from_utf8(password).unwrap()
            );
            eprintln!(
                "The encrypted password is {:?}",
                password_block.as_slice()
            );
            eprintln!("The name of the password is {}", name);
        }
        _ => {
            panic!("truly a bruh moment, this should be unreachable");
        }
    }
}
