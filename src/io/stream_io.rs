//! This module is for non-interactive Standard I/O.

use std::io::{
    self,
    Write,
};
use eyre::{
    eyre,
    Result,
};
use zeroize::Zeroizing;

use crate::string_codec;


/// Reads string from stdin and converts it into Zeroizing-wrapped UTF-8 bytes.
pub fn read_secret_from_stdin() -> Result<Zeroizing<Vec<u8>>> {
    let mut raw_secret = Zeroizing::new(String::new());
    io::stdin().read_line(&mut raw_secret)?;
    let secret_bytes = Zeroizing::new(string_codec::into_utf8_bytes(raw_secret.trim()));
    Ok(secret_bytes)
}


/// Writes the specified result into stdout.
pub fn write_to_stdout(value: &String) -> Result<()> {
    let mut out = io::stdout();
    writeln!(out, "{value}").map_err(|e| eyre!("{e:?}"))?;
    out.flush().map_err(|e| eyre!("{e:?}"))?;
    Ok(())
}
