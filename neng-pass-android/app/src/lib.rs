use jni::{
    objects::{JClass, JString},
    sys::{jarray, jboolean},
    JNIEnv,
};
use std::path::PathBuf;

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
) -> jarray {
    let string_class = env.find_class("java/lang/String").unwrap();
    let database_file = env
        .get_string(&p_database_file)
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let database = open_and_prepare_database(&PathBuf::from(database_file)).unwrap();

    let mut sql_statement = database.prepare("SELECT name FROM passwords").unwrap();

    let password_count = sql_statement.column_count();

    let mut passwords = env
        .new_object_array(
            password_count.try_into().unwrap(),
            string_class,
            env.new_string("").unwrap(),
        )
        .unwrap();

    for (i, row) in sql_statement.iter().enumerate() {
        let row = row.unwrap();
        let name: &str = row.read(0);

        env.set_object_array_element(
            &mut passwords,
            i.try_into().unwrap(),
            env.new_string(name).unwrap(),
        ).unwrap();
    }

    passwords.as_raw()
}
