use std::{
    fs::File,
    io::{Read, Write},
    string::FromUtf8Error,
};

pub mod crypto;

use argon2::{password_hash::Error as Argon2Error, Argon2, PasswordHash, PasswordVerifier};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use rusqlite::Error as SqliteError;
use std::io::Error as IOError;

pub use rusqlite;

pub const MAX_MASTER_KEY_LEN: usize = 32;
pub const MAX_PASSWORD_LEN: usize = 16;

#[derive(Debug)]
pub enum Error {
    HashError(Argon2Error),
    DatabaseError(SqliteError),
    IOError(IOError),
    FromUtf8Error(FromUtf8Error),
    MasterKeyDoesntExist,
    MasterKeyTooLong,
    MasterKeyAlreadyExists,
    PasswordAlreadyExists,
    PasswordDoesntExist(Box<str>),
    PasswordTooLong,
    UnknownError,
}

impl Error {
    pub fn get_message(&self) -> String {
        match self {
            Error::HashError(err) => match err {
                Argon2Error::Password => {
                    format!("The password is incorrect.")
                }
                _ => {
                    format!("Failed to hash the password: {}", err)
                }
            },
            Error::DatabaseError(err) => {
                format!("SQL error: {}", err)
            }
            Error::IOError(err) => {
                format!("IO Error: {}", err)
            }
            Error::FromUtf8Error(err) => {
                format!("Invalid UTF-8 string: {}", err)
            }
            Error::MasterKeyTooLong => {
                format!("Your master key is too long! Master keys can only be up to {} characters long.", MAX_MASTER_KEY_LEN)
            }
            Error::MasterKeyAlreadyExists => {
                format!("The master key has already been set. Don't try to set it again, as it will break stuff.")
            }
            Error::MasterKeyDoesntExist => {
                "It looks like you didn't set a master key yet! Use the set-master command to do so.".to_string()
            }
            Error::PasswordAlreadyExists => {
                format!("A password with that name already exists!")
            },
            Error::PasswordDoesntExist(name) => {
                format!("There appears to be no password saved that is named {}", name)
            },
            Error::PasswordTooLong => {
                format!("Your password is too long! Passwords can only be up to {} characters long.", MAX_PASSWORD_LEN)
            }
            Error::UnknownError => {
                "Sorry, but something went wrong.".to_string()
            }
        }
    }
}

impl From<Argon2Error> for Error {
    fn from(value: Argon2Error) -> Self {
        Self::HashError(value)
    }
}

impl From<SqliteError> for Error {
    fn from(value: SqliteError) -> Self {
        Self::DatabaseError(value)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: IOError) -> Self {
        Self::IOError(value)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(value: FromUtf8Error) -> Self {
        Self::FromUtf8Error(value)
    }
}

impl From<Error> for String {
    fn from(value: Error) -> Self {
        value.get_message()
    }
}

pub fn set_master_key(p_file: &str, p_new_key: &str) -> Result<(), Error> {
    if File::open(p_file).is_ok() {
        return Err(Error::MasterKeyAlreadyExists);
    }

    if p_new_key.len() > MAX_MASTER_KEY_LEN {
        return Err(Error::MasterKeyTooLong);
    }

    let new_key_hashed = crypto::hash(p_new_key)?;
    let mut file = File::create(p_file)?;
    file.write_all(&new_key_hashed)?;

    Ok(())
}

pub fn query_master_key(
    p_master_key_file: &str,
    p_inputted_password: &str,
) -> Result<String, Error> {
    let mut master_key_file = match File::open(p_master_key_file) {
        Ok(key) => key,
        Err(_) => {
            return Err(Error::MasterKeyDoesntExist);
        }
    };

    let mut actual_key_hashed = Vec::new();
    master_key_file.read_to_end(&mut actual_key_hashed)?;

    let actual_key_hashed = String::from_utf8(actual_key_hashed)?;

    let argon2 = Argon2::default();
    let actual_key_hashed = PasswordHash::new(&actual_key_hashed).unwrap();

    argon2.verify_password(p_inputted_password.as_bytes(), &actual_key_hashed)?;

    Ok(p_inputted_password.to_string())
}

pub fn generate_password() -> [u8; MAX_PASSWORD_LEN] {
    let mut password_generator = ChaCha20Rng::from_entropy();

    let mut password = [0u8; MAX_PASSWORD_LEN];

    password.iter_mut().for_each(|c| {
        *c = password_generator.gen_range(33..127);
    });

    password
}

pub fn create_password(
    mut p_master_key: String,
    p_name: &str,
    p_password: &str,
    p_sql_connection: &rusqlite::Connection,
) -> Result<(), Error> {
    let mut sql_statement = p_sql_connection.prepare("SELECT name FROM passwords WHERE name = ?")?;
    let password_names = sql_statement.query_map([p_name], |row| row.get::<_, String>(0))?;

    if password_names.count() > 0 {
        return Err(Error::PasswordAlreadyExists);
    }

    if p_password.len() > MAX_PASSWORD_LEN {
        return Err(Error::PasswordTooLong);
    }

    // Pad the master key

    while p_master_key.len() < MAX_MASTER_KEY_LEN {
        p_master_key.push(' ');
    }

    let encrypted_password = crypto::encrypt(p_master_key.as_bytes(), &p_password.as_bytes());

    p_sql_connection.execute(
        "INSERT INTO passwords VALUES (?, ?)",
        (p_name, &encrypted_password[..]),
    )?;

    Ok(()) // Placeholder
}

pub fn get_password(
    mut p_master_key: String,
    p_name: &str,
    p_sql_connection: &rusqlite::Connection,
) -> Result<String, Error> {
    while p_master_key.len() < MAX_MASTER_KEY_LEN {
        p_master_key.push(' ');
    }

    /* let sql_query = "SELECT * FROM passwords WHERE name = ?;";
    let mut sql_statement = p_sql_connection.prepare(sql_query)?;
    sql_statement.bind((1, p_name))?; */

    let mut sql_statement = p_sql_connection.prepare("SELECT * FROM passwords WHERE name = ?;")?;
    let mut password_names = sql_statement.query_map([p_name], |row| row.get::<_, Vec<u8>>(1))?;

    let row = match password_names.next() {
        Some(row) => row,
        None => return Err(Error::PasswordDoesntExist(Box::from(p_name))),
    }?;

    let password = row;
    let decrypted_password = crypto::decrypt(p_master_key.as_bytes(), &password);

    Ok(String::from_utf8_lossy(&decrypted_password).to_string())
}

pub fn delete_password(p_name: &str, p_sql_connection: &rusqlite::Connection) -> Result<(), Error> {
    let mut sql_statement = p_sql_connection.prepare("DELETE FROM passwords WHERE name = ?;")?;
    sql_statement.execute([p_name])?;
    Ok(())
}
