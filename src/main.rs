use std::{io::Write, path::Path};

use aes::cipher::{generic_array::GenericArray, BlockDecrypt, KeyInit};
use clap::{arg, Arg, ArgAction, Command};
use directories::ProjectDirs;

use neng_pass::{MAX_MASTER_KEY_LEN, MAX_PASSWORD_LEN};

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
        .subcommand(
            Command::new("get")
                .about("Gets the value of a specific password.")
                .arg(arg!(<NAME> "The name of the password that you want to get."))
                .arg(
                    Arg::new("raw")
                        .short('r')
                        .long("raw")
                        .action(ArgAction::SetTrue)
                        .help("Output as a raw output, to be piped into other commands."),
                ),
        )
        .subcommand(
            Command::new("list")
                .about("Gets a list of all the passwords that you have saved right now."),
        )
        .subcommand(
            Command::new("delete")
                .about("Deletes a specific password from your password list. Remember that this is not reversible!!!!")
                .arg(arg!(<NAME> "The name of the password that you want to delete."))
            )
}

fn ask_for_password(p_master_key_file: &str) -> String {
    let user_input_password = rpassword::prompt_password("Enter the master key: ").unwrap();
    match neng_pass::query_master_key(p_master_key_file, &user_input_password) {
        Ok(key) => key,
        Err(err) => {
            eprintln!("[ERROR]: {}", err.get_message());
            std::process::exit(1);
        }
    }
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

    let sql_connection = sqlite::open(format!("{}/passwords.db", data_dir)).unwrap();
    sql_connection
        .execute("CREATE TABLE IF NOT EXISTS passwords (name TEXT, password BLOB);")
        .unwrap();

    let master_key_path = format!("{}/master_key", data_dir);

    match cli_matches.subcommand() {
        Some(("set-master", _)) => {
            let new_key = rpassword::prompt_password("Enter a new master key: ").unwrap();
            let new_key_confirmation =
                rpassword::prompt_password("Confirm your master password: ").unwrap();
            if new_key != new_key_confirmation {
                eprintln!("The passwords you entered do not match!");
                std::process::exit(1);
            }

            if let Err(err) =
                neng_pass::set_master_key(format!("{}/master_key", data_dir).as_str(), &new_key)
            {
                eprintln!("[ERROR]: {}", err.get_message());
                std::process::exit(1);
            }

            eprintln!("Successfully updated the master key file.");
        }
        Some(("new", sub_matches)) => {
            let master_key = ask_for_password(&master_key_path);
            let name = sub_matches.get_one::<String>("NAME").unwrap();
            if let Err(err) = neng_pass::create_password(&master_key, &name, &sql_connection) {
                eprintln!("[ERROR]: {}", err.get_message());
                std::process::exit(1);
            }

            eprintln!("Created and saved password named '{}'", name);
        }
        Some(("get", sub_matches)) => {
            let mut master_key = ask_for_password(&master_key_path);

            while master_key.len() < MAX_MASTER_KEY_LEN {
                master_key.push(' ');
            }

            let mut master_key_block = [b' '; MAX_MASTER_KEY_LEN];
            master_key_block.copy_from_slice(master_key.as_bytes());
            let master_key_block = GenericArray::from(master_key_block);
            let name = sub_matches.get_one::<String>("NAME").unwrap();

            let sql_query = "SELECT * FROM passwords WHERE name = ?;";
            let mut sql_statement = sql_connection.prepare(sql_query).unwrap();
            sql_statement.bind((1, name.as_str())).unwrap();

            let first_row = match match sql_statement.iter().next() {
                Some(row) => row,
                None => {
                    eprintln!("That password doesn't exist, idiot.");
                    std::process::exit(1);
                }
            } {
                Ok(row) => row,
                Err(err) => {
                    eprintln!("Can't get that password. Error {}", err);
                    std::process::exit(1);
                }
            };

            let password_blob: &[u8] = first_row.read(1);
            let mut password_block = [0u8; MAX_PASSWORD_LEN];
            password_block.copy_from_slice(password_blob);
            let mut password_block = GenericArray::from(password_block);

            let cipher = aes::Aes256::new(&master_key_block);
            cipher.decrypt_block(&mut password_block);

            let raw_mode = sub_matches.get_flag("raw");
            if raw_mode {
                std::io::stdout().write(password_block.as_slice()).unwrap();
            } else {
                eprintln!(
                    "Here's the password: {}",
                    String::from_utf8(password_block.as_slice().to_vec()).unwrap()
                );
            }
        }
        Some(("list", _)) => {
            ask_for_password(&master_key_path);

            let sql_query = "SELECT name FROM passwords;";
            let mut sql_statement = sql_connection.prepare(sql_query).unwrap();

            eprintln!("Here is the list of passwords that you have stored.\n");

            sql_statement
                .iter()
                .map(|row| row.unwrap())
                .for_each(|row| {
                    let name: &str = row.read(0);
                    eprintln!("\t - {}", name);
                });
        }
        Some(("delete", sub_matches)) => {
            ask_for_password(&master_key_path);

            let name: &String = sub_matches.get_one("NAME").unwrap();

            let sql_query = "DELETE FROM passwords WHERE name = ?";
            let mut sql_statement = sql_connection.prepare(sql_query).unwrap();
            sql_statement.bind((1, name.as_str())).unwrap();

            sql_statement.iter().for_each(|_| {});

            eprintln!("I have deleted all the passwords named '{}'", name);
        }
        _ => {
            panic!("truly a bruh moment, this should be unreachable");
        }
    }
}
