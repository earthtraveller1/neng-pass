use std::{fs::File, io::Write};

pub mod crypto;

use argon2::password_hash::Error as Argon2Error;
use std::io::Error as IOError;

pub const MAX_MASTER_KEY_LEN: usize = 32;

pub enum Error {
    HashError(Argon2Error),
    IOError(IOError),
    MasterKeyTooLong,
    MasterKeyAlreadyExists,
}

impl Error {
    pub fn get_message(&self) -> String {
        match self {
            Error::HashError(err) => {
                format!("Failed to hash the password: {}", err)
            }
            Error::IOError(err) => {
                format!("IO Error: {}", err)
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

impl From<std::io::Error> for Error {
    fn from(value: IOError) -> Self {
        Self::IOError(value)
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
