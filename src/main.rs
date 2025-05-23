//! # sphragis
//! Deterministic BIP-39 passphrase derivation tool.

mod initialization;
mod commands;
mod config;
mod string_codec;
mod io;
mod crypto;


use std::{
    path::{
        Path,
        PathBuf
    },
    process::ExitCode,
};
use clap::{
    Parser,
    Subcommand,
    Args,
};
use tracing::error;


/// Struct that represents parsed runtime arguments.
#[derive(Parser)]
#[command(name = "sphragis")]
#[command(about = "Memory-based BIP-39 passphrase derivation tool", long_about = None)]
#[command(subcommand_required = false)]
pub struct Cli {
    /// Subcommand to run
    #[command(subcommand)]
    pub command: Option<Command>,

    /// Enable verbose output
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub verbose: bool,
}

impl Cli {
    pub fn log_level(&self) -> tracing::Level {
        if self.verbose {
            tracing::Level::DEBUG
        } else {
            tracing::Level::INFO
        }
    }
}

const DEFAULT_CONFIG_FILE_PATH: &str = "sphragis.json5";
const DEFAULT_DISPLAY_DURATION: u64 = 60;
const DEFAULT_SALT_LENGTH: usize = 16;

/// Arguments specific to the `Command::Derive` subcommand.
#[derive(Args)]
pub struct DeriveArgs {
    /// Path to the configuration file
    #[arg(short, long, default_value = DEFAULT_CONFIG_FILE_PATH)]
    pub config: PathBuf,

    /// Duration in seconds to display the final BIP-39 passphrase
    #[arg(long, default_value_t = DEFAULT_DISPLAY_DURATION)]
    pub display_duration: u64,
}

impl Default for DeriveArgs {
    fn default() -> Self {
        Self {
            config: Path::new(DEFAULT_CONFIG_FILE_PATH).into(),
            display_duration: DEFAULT_DISPLAY_DURATION,
        }
    }
}

/// Subcommands.
#[derive(Subcommand)]
pub enum Command {
    /// Derive a BIP-39 passphrase from user-entered secret.
    Derive(DeriveArgs),
    /// Generate a new random salt (base64).
    GenerateSalt {
        /// Number of bytes to generate.
        #[arg(short, long, default_value_t = DEFAULT_SALT_LENGTH)]
        length: usize,
    },
}


/// Process entry point:
/// Parses runtime arguments, dispatches subcommand, and converts `eyre::Result<_>` into `std::process::ExitCode`.
fn main() -> ExitCode {
    let cli = Cli::parse();
    //✅ Setup logging according to "verbose" flag.
    initialization::setup_logging(module_path!(), cli.log_level());

    //💡 Subcommand defaults to Derive.
    let subcommand = cli.command.unwrap_or(Command::Derive(DeriveArgs::default()));
    //✅ Execute subcommand.
    match commands::run(&subcommand) {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            error!("{e:?}");
            ExitCode::FAILURE
        },
    }
}
