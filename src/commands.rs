//! Handles subcommand dispatch and execution logic.
//!
//! This module defines the high-level interface for matching and running
//! individual subcommands based on the parsed CLI input.
//!
//! This is the central hub for orchestrating command-line operations.

use std::time::Instant;
use eyre::Result;
use tracing::debug;
use zeroize::Zeroizing;
use atty::Stream;


use crate::{
    config::{
        self,
        Argon2idContext,
    },
    crypto,
    string_codec,
    io::{
        stream_io,
        terminal_io,
    },
    Command,
    DeriveArgs,
};


/// Executes the specified subcommand and returns `eyre::Result<()>`.
pub fn run(command: &Command) -> Result<()> {
    //✅ Subcommand dispatching.
    match command {
        Command::Derive(args) => run_derive(&args),
        Command::GenerateSalt {length} => run_generate_salt(*length),
    }
}


/// Executes BIP-39 passphrase derivation.
fn run_derive(args: &DeriveArgs) -> Result<()> {
    //✅ Load non-secret Argon2id configuration from file.
    let config_file_path = args.config.as_path();
    debug!("Reading config from {config_file_path:?}");
    let argon2id_ctx = match config::load_from_path(config_file_path) {
        Ok(ctx) => ctx,
        Err(e) => return Err(e),
    };
    debug!("Using Argon2id version={:?} with params={:?}", argon2id_ctx.version, argon2id_ctx.params);
    debug!("Using salt: {}", string_codec::hex_string_from_bytes(&argon2id_ctx.salt));

    //✅ Generate and display BIP-39 passphrase.
    if atty::is(Stream::Stdin) {
        //🔶 interactive mode.
        generate_and_display_passphrase_interractively(args, argon2id_ctx)
    } else {
        //🔶 batch mode.
        generate_and_output_passphrase_in_batch_mode(args, argon2id_ctx)
    }

}


fn generate_and_display_passphrase_interractively(args: &DeriveArgs, argon2id_ctx: Argon2idContext) -> Result<()> {
    //✅ Acquire secret bytes interactively from user.
    //📌 The secret bytes are Zeroizing-wrapped.
    let secret: Zeroizing<Vec<u8>> = match terminal_io::obtain_verified_secret() {
        Ok(bytes) => bytes,
        Err(e) => return Err(e),
    };

    //✅ Derive argon2id key bytes from secret and salt.
    //📌 Derived key is Zeroizing-wrapped.
    let stime = Instant::now();
    let key_derivation_result: Result<Zeroizing<[u8; 32]>>  = terminal_io::with_spinner(
        || crypto::derive_key(&secret, &argon2id_ctx.salt, argon2id_ctx.version, argon2id_ctx.params)
    );
    let elapsed = stime.elapsed();
    match key_derivation_result  {
        Ok(derived_key) => {
            debug!("Key derivation by Argon2id completed in {} ms.", elapsed.as_millis());
            //✅ Stringify the derived key into BIP-39 passphrase.
            //📌 The passphrase is Zeroizing-wrapped.
            let bip39_passphrase: Zeroizing<String> = string_codec::passphrase_from_key(derived_key);
            //✅ Display the BIP-39 passphrase for a limited duration.
            terminal_io::show_transient_output(&bip39_passphrase, args.display_duration)?;
            Ok(())
        },
        Err(e) => Err(e),
    }
}


fn generate_and_output_passphrase_in_batch_mode(_args: &DeriveArgs, argon2id_ctx: Argon2idContext) -> Result<()> {
    //✅ Read secret bytes from stdin.
    //📌 The secret bytes are Zeroizing-wrapped.
    let secret: Zeroizing<Vec<u8>> = match stream_io::read_secret_from_stdin() {
        Ok(bytes) => bytes,
        Err(e) => return Err(e),
    };

    //✅ Derive argon2id key bytes from secret and salt.
    //📌 Derived key is Zeroizing-wrapped.
    let stime = Instant::now();
    let key_derivation_result: Result<Zeroizing<[u8; 32]>>  =
        crypto::derive_key(&secret, &argon2id_ctx.salt, argon2id_ctx.version, argon2id_ctx.params);
    let elapsed = stime.elapsed();
    match key_derivation_result  {
        Ok(derived_key) => {
            debug!("Key derivation by Argon2id completed in {} ms.", elapsed.as_millis());
            //✅ Stringify the derived key into BIP-39 passphrase.
            //📌 The passphrase is Zeroizing-wrapped.
            let bip39_passphrase: Zeroizing<String> = string_codec::passphrase_from_key(derived_key);
            //✅ Output the BIP-39 passphrase.
            stream_io::write_to_stdout(&bip39_passphrase)
        },
        Err(e) => Err(e),
    }
}


/// Executes salt generation.
fn run_generate_salt(length: usize) -> Result<()> {
    //✅ Generate salt of specified number of bytes.
    let salt = crypto::generate_salt(length)?;
    //✅ Stringify the salt bytes by Base64 encoding.
    let salt_string = string_codec::base64_string_from_salt(&salt);
    //✅ Output the encoded string to stdout.
    stream_io::write_to_stdout(&salt_string)
}
