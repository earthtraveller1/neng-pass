use aes::cipher::{generic_array::GenericArray, BlockDecrypt, BlockEncrypt, KeyInit};
use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;

use crate::{MAX_MASTER_KEY_LEN, MAX_PASSWORD_LEN};

pub fn hash(p_input: &str) -> argon2::password_hash::Result<Vec<u8>> {
    let mut cha_cha_rng = ChaCha20Rng::from_entropy();
    let salt = SaltString::generate(&mut cha_cha_rng);
    let argon2 = Argon2::default();
    Ok(argon2
        .hash_password(p_input.as_bytes(), &salt)?
        .to_string()
        .as_bytes()
        .to_owned())
}

pub fn encrypt(p_key: &[u8], p_data: &[u8]) -> [u8; MAX_PASSWORD_LEN] {
    let mut key = [0u8; MAX_MASTER_KEY_LEN];
    key.copy_from_slice(p_key);
    let key_block = GenericArray::from(key);

    let mut data = [0u8; MAX_PASSWORD_LEN];
    data.copy_from_slice(p_data);
    let mut data_block = GenericArray::from(data);

    let cipher = aes::Aes256::new(&key_block);
    cipher.encrypt_block(&mut data_block);
    data.copy_from_slice(data_block.as_slice());

    data
}

pub fn decrypt(p_key: &[u8], p_data: &[u8]) -> [u8; MAX_PASSWORD_LEN] {
    let mut key = [0u8; MAX_MASTER_KEY_LEN];
    key.copy_from_slice(p_key);
    let key_block = GenericArray::from(key);

    let mut data = [0u8; MAX_PASSWORD_LEN];
    data.copy_from_slice(p_data);
    let mut data_block = GenericArray::from(data);

    let cipher = aes::Aes256::new(&key_block);
    cipher.decrypt_block(&mut data_block);
    data.copy_from_slice(data_block.as_slice());

    data
}
