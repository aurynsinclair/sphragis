//! This module is for reading, parsing and verifying configuration file.


use std::{
    fs,
    path::Path,
};
use eyre::{
    eyre,
    Result,
};

use serde::Deserialize;
use json5;

use argon2::{
    Version,
    Params
};

use crate::string_codec;


#[derive(Debug, Deserialize)]
struct Argon2idParams {
    m_cost: u32,
    t_cost: u32,
    p_cost: u32,
}

#[derive(Debug, Deserialize)]
struct RawConfig {
    version: String,
    params: Argon2idParams,
    salt: String,
}


/// Non-secret inputs for Argon2id key derivation function.
#[derive(Debug, Clone)]
pub struct Argon2idContext {
    pub version: Version,
    pub params: Params,
    pub salt: Vec<u8>,
}


/// Tries to read, parse, and verify the non-secret inputs for the BIP-39 passphrase derivation from the specified path.
/// Returns (argon2 version, argon2id params, salt bytes) if successfull.
pub fn load_from_path(path: &Path) -> Result<Argon2idContext> {
    let raw_config = load_config(path)?;
    let version = match raw_config.version.as_str() {
        "0x10" => {
            Version::V0x10
        },
        "0x13" => {
            Version::V0x13
        },
        _ => {
            return Err(eyre!("Version must be eitehr 0x10 or 0x13: {}", raw_config.version));
        }
    };
    let params = Params::new(
        raw_config.params.m_cost,
        raw_config.params.t_cost,
        raw_config.params.p_cost,
        None,
    ).map_err(|e| eyre!("{e:?}"))?;
    let salt = parse_salt(&raw_config.salt)?;

    Ok(Argon2idContext { version, params, salt })
}


/// Tries to read the file body from the specified path and parse it as JSON5.
/// Returns the parsed struct if successfull.
fn load_config(path: &Path) -> Result<RawConfig> {
    let raw = fs::read_to_string(path)?;
    let cfg: RawConfig = json5::from_str(&raw)?;
    Ok(cfg)
}


/// Tries to parse a salt string as base64.
/// Returns raw bytes if successful.
fn parse_salt(s: &str) -> Result<Vec<u8>> {
    if let Ok(bytes) = string_codec::parse_base64_str(s) {
        Ok(bytes)
    } else {
        Err(eyre!("Failed to parse string ({} chars). Expected base64 or hex-encoded string.", s.len()))
    }
}
