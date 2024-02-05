// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::PathBuf;

use directories::ProjectDirs;

struct InternalState {
    master_key: Option<String>,
}

fn get_data_dir() -> PathBuf {
    match ProjectDirs::from("io", "earthtraveller1", "neng-pass") {
        Some(project_dirs) => project_dirs.data_dir().to_owned(),
        None => PathBuf::from("./.neng-pass"),
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
fn get_password_list() -> Result<Vec<String>, String> {
    let data_dir = get_data_dir();
    let sql_connection = open_and_prepare_database(&data_dir)?;

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
fn create_password(p_master_key: &str, p_name: &str) -> Result<(), String> {
    let data_dir = get_data_dir();
    let sql_connection = open_and_prepare_database(&data_dir)?;

    match neng_pass::create_password(p_master_key.to_string(), p_name, &sql_connection) {
        Ok(_) => Ok(()),
        Err(err) => Err(err.get_message()),
    }
}

#[tauri::command]
async fn is_master_key_correct(p_master_key: &str) -> Result<bool, ()> {
    let mut data_dir = get_data_dir();
    data_dir.push("master_key");

    Ok(neng_pass::query_master_key(data_dir.to_str().unwrap(), p_master_key).is_ok())
}

fn main() {
    tauri::Builder::default()
        .manage(InternalState { master_key: None })
        .invoke_handler(tauri::generate_handler![
            get_password_list,
            create_password,
            is_master_key_correct
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
