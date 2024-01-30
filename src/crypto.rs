use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;

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
