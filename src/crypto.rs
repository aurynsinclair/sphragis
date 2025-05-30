//! This module contains cryptography-related functions.

use eyre::{
    eyre,
    Result
};
use zeroize::Zeroizing;
use argon2::{
    Argon2,
    Algorithm,
    Version,
    Params,
};
use rand_core::{
    OsRng,
    TryRngCore,
};


/// Generates salt of the specified length.
/// Simply returns the generated bytes if successfull.
pub fn generate_salt(length: usize) -> Result<Vec<u8>> {
    let mut bytes = vec![0_u8; length];
    OsRng.try_fill_bytes(&mut bytes).map_err(|e| eyre!{"{e:?}"})?;
    Ok(bytes)
}


/// Derives argon2id key from specified inputs.
/// Returns the output as Zeroizing-wrapped bytes if successfull.
pub fn derive_key(
    secret: &[u8],
    salt: &[u8],
    version: Version,
    params: Params,
) -> Result<Zeroizing<[u8; 32]>> {
    let password_hasher = Argon2::new(
        Algorithm::Argon2id,
        version,
        params,
    );
    let mut key_buf = Zeroizing::new([0u8; 32]);
    password_hasher
        .hash_password_into(secret, salt, &mut *key_buf)
        .map_err(|e| eyre!("{e:?}"))?;
    Ok(key_buf)
}
