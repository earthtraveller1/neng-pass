use jni::{
    objects::{JClass, JString},
    sys::{jboolean, jobjectArray, jstring},
    JNIEnv,
};

use neng_pass::rusqlite;

use std::path::PathBuf;

fn open_and_prepare_database(data_dir: &PathBuf) -> Result<rusqlite::Connection, String> {
    let sql_connection = match rusqlite::Connection::open(data_dir.join("passwords.db")) {
        Ok(connection) => connection,
        Err(err) => return Err(format!("Failed to open the database: {}", err)),
    };

    if let Err(err) = sql_connection.execute(
        "CREATE TABLE IF NOT EXISTS passwords (name TEXT, password BLOB);",
        (),
    ) {
        return Err(format!(
            "Failed to prepare the table in the database: {}",
            err
        ));
    }

    Ok(sql_connection)
}

#[no_mangle]
//                     Java_io_github_earthtraveller1_nengpass_NengPass_00024Companion_setMasterKey
pub extern "system" fn Java_io_github_earthtraveller1_nengpass_NengPass_00024Companion_setMasterKey(
    mut env: JNIEnv,
    _p_class: JClass,
    p_file: JString,
    p_master_key: JString,
) {
    let file_name = env
        .get_string(&p_file)
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    let master_key = env
        .get_string(&p_master_key)
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    neng_pass::set_master_key(&file_name, &master_key).unwrap();
}

#[no_mangle]
pub extern "system" fn Java_io_github_earthtraveller1_nengpass_NengPass_00024Companion_isMasterKeyCorrect(
    mut env: JNIEnv,
    _p_class: JClass,
    p_file: JString,
    p_master_key: JString,
) -> jboolean {
    // android_log::init("io.github.earthtraveller1.nengpass").unwrap();
    // log_panics::init();

    let file_name = env
        .get_string(&p_file)
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    let master_key = env
        .get_string(&p_master_key)
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    neng_pass::query_master_key(&file_name, &master_key).is_ok() as jboolean
}

#[no_mangle]
pub extern "system" fn Java_io_github_earthtraveller1_nengpass_NengPass_00024Companion_getPasswordList(
    mut env: JNIEnv,
    _p_class: JClass,
    p_database_file: JString,
) -> jobjectArray {
    let string_class = env.find_class("java/lang/String").unwrap();
    let database_file = env
        .get_string(&p_database_file)
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    log::debug!("[RUST]: Database file: {}", database_file);

    let database = open_and_prepare_database(&PathBuf::from(database_file)).unwrap();

    let mut sql_statement = database.prepare("SELECT name FROM passwords;").unwrap();

    let mut native_passwords = Vec::<String>::new();

    for row in sql_statement
        .query_map([], |row| row.get::<_, String>(0))
        .unwrap()
    {
        let row = row.unwrap();
        native_passwords.push(row);
    }

    let mut passwords = env
        .new_object_array(
            native_passwords.len().try_into().unwrap(),
            string_class,
            env.new_string("").unwrap(),
        )
        .unwrap();

    for (index, password) in native_passwords.iter().enumerate() {
         env.set_object_array_element(
             &mut passwords,
             index.try_into().unwrap(),
             env.new_string(password).unwrap(),
         )
         .unwrap();
    }

    passwords.as_raw()
}

#[no_mangle]
pub extern "system" fn Java_io_github_earthtraveller1_nengpass_NengPass_00024Companion_generatePassword(
    env: JNIEnv,
    _p_class: JClass,
) -> jstring {
    let password = std::str::from_utf8(&neng_pass::generate_password())
        .unwrap()
        .to_string();
    env.new_string(password).unwrap().as_raw()
}

#[no_mangle]
pub extern "system" fn Java_io_github_earthtraveller1_nengpass_NengPass_00024Companion_savePassword(
    mut env: JNIEnv,
    _p_class: JClass,
    p_database_file: JString,
    p_master_key: JString,
    p_name: JString,
    p_password: JString,
) {
    let database_file = env
        .get_string(&p_database_file)
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let database = open_and_prepare_database(&PathBuf::from(database_file)).unwrap();

    let master_key = env
        .get_string(&p_master_key)
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let name = env
        .get_string(&p_name)
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let password = env
        .get_string(&p_password)
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    neng_pass::create_password(master_key, &name, &password, &database).unwrap();
}
