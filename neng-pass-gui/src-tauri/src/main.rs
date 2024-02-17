// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{path::PathBuf, sync::Mutex, fs::File};

use directories::ProjectDirs;

struct InternalState {
    master_key: Option<String>,
}

struct StaticState {
    data_dir: PathBuf,
}

struct State {
    internal_state: Mutex<InternalState>,
    static_state: StaticState,
}

impl State {
    fn new() -> State {
        State {
            internal_state: Mutex::new(InternalState { master_key: None }),
            static_state: StaticState {
                data_dir: match ProjectDirs::from("io", "earthtraveller1", "neng-pass") {
                    Some(project_dirs) => project_dirs.data_dir().to_owned(),
                    None => PathBuf::from("./.neng-pass"),
                },
            },
        }
    }
}

fn open_and_prepare_database(data_dir: &PathBuf) -> Result<sqlite::Connection, String> {
    let sql_connection = match sqlite::open(data_dir.join("passwords.db")) {
        Ok(connection) => connection,
        Err(err) => return Err(format!("Failed to open the database: {}", err)),
    };

    if let Err(err) =
        sql_connection.execute("CREATE TABLE IF NOT EXISTS passwords (name TEXT, password BLOB);")
    {
        return Err(format!(
            "Failed to prepare the table in the database: {}",
            err
        ));
    }

    Ok(sql_connection)
}

#[tauri::command]
fn get_password_list(p_state: tauri::State<'_, State>) -> Result<Vec<String>, String> {
    let sql_connection = open_and_prepare_database(&p_state.static_state.data_dir)?;

    let mut sql_statement = match sql_connection.prepare("SELECT name FROM passwords") {
        Err(err) => {
            return Err(format!(
                "Failed to query the database for a list of passwords: {}",
                err
            ))
        }
        Ok(statement) => statement,
    };

    let mut password_names = Vec::new();

    for row in sql_statement.iter() {
        match row {
            Ok(row) => {
                let name: &str = row.read(0);
                password_names.push(name.to_string());
            }
            Err(err) => {
                return Err(format!("Failed to read a row from the database: {}", err));
            }
        }
    }

    Ok(password_names)
}

#[tauri::command]
fn create_password(p_state: tauri::State<'_, State>, p_master_key: &str, p_name: &str) -> Result<(), String> {
    let sql_connection = open_and_prepare_database(&p_state.static_state.data_dir)?;

    match neng_pass::create_password(p_master_key.to_string(), p_name, &sql_connection) {
        Ok(_) => Ok(()),
        Err(err) => Err(err.get_message()),
    }
}

#[tauri::command]
async fn is_master_key_correct(p_state: tauri::State<'_, State>, p_master_key: &str) -> Result<bool, ()> {
    let mut data_dir = p_state.static_state.data_dir.clone();
    data_dir.push("master_key");

    Ok(neng_pass::query_master_key(data_dir.to_str().unwrap(), p_master_key).is_ok())
}

#[tauri::command]
async fn set_master_key(
    p_master_key: &str,
    p_state: tauri::State<'_, State>,
) -> Result<(), String> {
    (match p_state.internal_state.lock() {
        Ok(key) => key,
        Err(_) => return Err("Failed to acquire a lock for the master key.".to_string()),
    })
    .master_key = Some(p_master_key.to_string());

    Ok(())
}

#[tauri::command]
fn is_master_key_set(p_state: tauri::State<'_, State>) -> bool {
    match File::open(&p_state.static_state.data_dir) {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn main() {
    tauri::Builder::default()
        .manage(State::new())
        .invoke_handler(tauri::generate_handler![
            get_password_list,
            create_password,
            is_master_key_correct,
            set_master_key,
            is_master_key_set,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
