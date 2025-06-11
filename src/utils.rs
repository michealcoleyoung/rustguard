use argon2::{Argon2, Params};
use chacha20poly1305::Key;

pub fn derive_key(master_password: &str, salt: &[u8]) -> [u8; 32] {
    let mut key = [0u8; 32];
    let params = Params::new(19456, 2, 1, None).unwrap();
    let argon2 = Argon2::new(Default::default(), Default::default(), params);
    argon2.hash(&mut key, master_password.as_bytes(), salt, &[], &[]);
    key
}
