use sha2::{Sha512, Digest};

pub fn hash(p_input: &str) -> Vec<u8> {
    Vec::from(Sha512::digest(p_input).as_slice())
}

