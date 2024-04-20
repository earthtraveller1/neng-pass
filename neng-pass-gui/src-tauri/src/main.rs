// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fs::File, path::PathBuf, sync::Mutex};

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
        // Ensure that the data directory is created properly.
        let data_dir = match ProjectDirs::from("io", "earthtraveller1", "neng-pass") {
            Some(project_dirs) => project_dirs.data_dir().to_owned(),
            None => PathBuf::from("./.neng-pass"),
        };

        std::fs::create_dir_all(&data_dir).unwrap();

        State {
            internal_state: Mutex::new(InternalState { master_key: None }),
            static_state: StaticState { data_dir },
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
fn generate_password() -> Result<String, ()> {
    std::str::from_utf8(&neng_pass::generate_password())
        .map(|s| s.to_string())
        .map_err(|_| ())
}

#[tauri::command]
fn save_password(
    p_state: tauri::State<'_, State>,
    p_name: &str,
    p_password: &str,
) -> Result<(), String> {
    let sql_connection = open_and_prepare_database(&p_state.static_state.data_dir)?;
    let internal_state = p_state.internal_state.lock().unwrap();

    let master_key = match &internal_state.master_key {
        Some(master_key) => master_key,
        None => return Err("The master has not been set yet!".to_string()),
    };

    match neng_pass::create_password(master_key.clone(), p_name, p_password, &sql_connection) {
        Ok(_) => Ok(()),
        Err(err) => Err(err.get_message()),
    }
}

#[tauri::command]
async fn is_master_key_correct(
    p_state: tauri::State<'_, State>,
    p_master_key: &str,
) -> Result<bool, ()> {
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
async fn get_password(p_name: &str, p_state: tauri::State<'_, State>) -> Result<String, String> {
    let internal_state = p_state.internal_state.lock().unwrap();
    let master_key = internal_state
        .master_key
        .clone()
        .ok_or("The master key has not been set!")?;

    let sql_connection = open_and_prepare_database(&p_state.static_state.data_dir)?;
    let password = neng_pass::get_password(master_key, p_name, &sql_connection)?;

    Ok(password)
}

#[tauri::command]
async fn delete_password(p_name: &str, p_state: tauri::State<'_, State>) -> Result<(), String> {
    let sql_connection = open_and_prepare_database(&p_state.static_state.data_dir)?;
    neng_pass::delete_password(p_name, &sql_connection)?;

    Ok(())
}

#[tauri::command]
fn is_master_key_set(p_state: tauri::State<'_, State>) -> bool {
    let mut master_key_path = p_state.static_state.data_dir.clone();
    master_key_path.push("master_key");

    match File::open(&master_key_path) {
        Ok(_) => true,
        Err(_) => false,
    }
}

#[tauri::command]
fn set_new_master_key(
    p_state: tauri::State<'_, State>,
    p_new_master_key: &str,
) -> Result<(), String> {
    let mut master_key_path = p_state.static_state.data_dir.clone();
    master_key_path.push("master_key");

    neng_pass::set_master_key(
        master_key_path
            .to_str()
            .ok_or(neng_pass::Error::UnknownError)?,
        p_new_master_key,
    )?;

    Ok(())
}

fn main() {
    tauri::Builder::default()
        .manage(State::new())
        .invoke_handler(tauri::generate_handler![
            delete_password,
            generate_password,
            get_password,
            get_password_list,
            is_master_key_correct,
            is_master_key_set,
            save_password,
            set_master_key,
            set_new_master_key,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
