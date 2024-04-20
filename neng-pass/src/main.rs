use std::{io::Write, path::Path};

use clap::{arg, Arg, ArgAction, Command};
use directories::ProjectDirs;

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
            Command::new("save")
                .about("Saves a new password with the specified name and value.")
                .arg(arg!(<NAME> "The name to assign to the password."))
                .arg(arg!(<PASSWORD> "The value to assign to the password.")),
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

            if let Err(err) = neng_pass::create_password(master_key, &name, std::str::from_utf8(&neng_pass::generate_password()).unwrap(), &sql_connection) {
                eprintln!("[ERROR]: {}", err.get_message());
                std::process::exit(1);
            }

            eprintln!("Created and saved password named '{}'", name);
        }
        Some(("get", sub_matches)) => {
            let master_key = ask_for_password(&master_key_path);
            let name = sub_matches.get_one::<String>("NAME").unwrap();
            let decrypted_password =
                match neng_pass::get_password(master_key, &name, &sql_connection) {
                    Ok(password) => password,
                    Err(err) => {
                        eprintln!("[ERROR]: {}", err.get_message());
                        std::process::exit(1);
                    }
                };

            let raw_mode = sub_matches.get_flag("raw");
            if raw_mode {
                std::io::stdout()
                    .write(decrypted_password.as_bytes())
                    .unwrap();
            } else {
                eprintln!("Here's the password: {}", decrypted_password);
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

            if let Err(err) = neng_pass::delete_password(name.as_str(), &sql_connection) {
                eprintln!("[ERROR]: {}", err.get_message());
                std::process::exit(1);
            }

            eprintln!("I have deleted all the passwords named '{}'", name);
        }
        _ => {
            panic!("truly a bruh moment, this should be unreachable");
        }
    }
}
