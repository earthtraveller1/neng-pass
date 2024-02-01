use std::{
    fs::File,
    io::{Read, Write},
    string::FromUtf8Error,
};

pub mod crypto;

use argon2::{password_hash::Error as Argon2Error, Argon2, PasswordHash, PasswordVerifier};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use std::io::Error as IOError;
use sqlite::Error as SqliteError;

pub const MAX_MASTER_KEY_LEN: usize = 32;
pub const MAX_PASSWORD_LEN: usize = 16;

pub enum Error {
    HashError(Argon2Error),
    DatabaseError(SqliteError),
    IOError(IOError),
    FromUtf8Error(FromUtf8Error),
    MasterKeyTooLong,
    MasterKeyAlreadyExists,
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
            }
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
    let mut master_key_file = File::open(p_master_key_file)?;
    let mut actual_key_hashed = Vec::new();
    master_key_file.read_to_end(&mut actual_key_hashed)?;

    let actual_key_hashed = String::from_utf8(actual_key_hashed)?;

    let argon2 = Argon2::default();
    let actual_key_hashed = PasswordHash::new(&actual_key_hashed).unwrap();

    argon2.verify_password(p_inputted_password.as_bytes(), &actual_key_hashed)?;

    Ok(p_inputted_password.to_string())
}

pub fn create_password(p_master_key: &str, p_name: &str, p_sql_connection: &sqlite::Connection) -> Result<String, Error> {
    let generated_password = {
        let mut random_generator = ChaCha20Rng::from_entropy();
        let mut password = [0u8; MAX_PASSWORD_LEN];

        password.iter_mut().for_each(|c| {
            *c = random_generator.gen_range(33..127);
        });

        password
    };

    // Pad the master key
    let master_key = {
        let mut master_key = p_master_key.to_owned();

        while master_key.len() < MAX_MASTER_KEY_LEN {
            master_key.push(' ');
        }

        master_key
    };

    let encrypted_password = crypto::encrypt(master_key.as_bytes(), &generated_password);
    
    let sql_query = "INSERT INTO passwords VALUES (?, ?)";

    let mut sql_statement = p_sql_connection.prepare(sql_query)?;
    sql_statement.bind((1, p_name))?;
    sql_statement.bind((2, &encrypted_password[..]))?;

    // This is how you run SQLite statements, apparently.
    sql_statement.iter().for_each(|_| {});

    Ok(String::new()) // Placeholder
}
