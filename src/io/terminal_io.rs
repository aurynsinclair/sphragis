//! This module is for interactive I/O on screen.


use std::{
    thread,
    time::Duration,
    io::{
        self,
        Stdout,
        Write,
    },
    sync::mpsc,
};

use eyre::{
    eyre,
    Result,
};
use zeroize::Zeroizing;

use crossterm::{ExecutableCommand, terminal, cursor};
use indicatif::{ProgressBar, ProgressStyle};

use crate::string_codec;


/// Prompt the user visibly and allow review before accepting the secret phrase.
/// The secret is returned as a Zeroizing-wrapped byte vector, already trimmed and encoded.
pub fn obtain_verified_secret() -> Result<Zeroizing<Vec<u8>>> {
    let mut out = io::stdout();

    clear_terminal(&mut out)?;
    // Let user enter the secret string.
    let raw_secret = prompt_for_secret(&mut out)?;
    let trimmed_secret = Zeroizing::new(raw_secret.trim().to_string());
    let confirmation = confirm_secret(&mut out, &trimmed_secret)?;
    clear_terminal(&mut out)?;

    match confirmation.trim().to_lowercase().as_str() {
        "" | "y" | "yes" => {
            //🔶 User accepted (including default by pressing ENTER).
            let secret_bytes = Zeroizing::new(string_codec::into_utf8_bytes(&trimmed_secret));
            Ok(secret_bytes)
        },
        _ => {
            //🔶 User rejected.
            Err(eyre!("Secret entry aborted by user."))
        },
    }
}

fn prompt_for_secret(out: &mut Stdout) -> Result<Zeroizing<String>> {
    writeln!(out, "\n================ Secret Entry =================\n")?;
    writeln!(out, "Before proceeding, ensure that you are alone and no one is observing your screen.")?;
    writeln!(out)?;
    writeln!(out, "You will now enter your secret phrase. For accuracy, it will be displayed as you type.")?;
    writeln!(out, "The screen will be cleared after input is complete.")?;
    writeln!(out, "Type your secret phrase and press Enter when finished.")?;
    writeln!(out)?;
    write!(out, "> ")?;
    out.flush()?;

    let mut raw_input = Zeroizing::new(String::new());
    io::stdin().read_line(&mut raw_input)?;
    Ok(raw_input)
}

fn confirm_secret(out: &mut Stdout, secret: &str) -> Result<String> {
    writeln!(out)?;
    writeln!(out, "\n================ Secret Confirmation =================\n")?;
    writeln!(out, "The following secret will be used to derive your BIP-39 passphrase:\n")?;

    writeln!(out, "{secret}")?;
    writeln!(out)?;
    writeln!(out, "[Length: {} characters]\n", secret.chars().count())?;

    writeln!(out, "Please double-check the above.")?;
    writeln!(out, "    - Leading/trailing whitespaces have been removed automatically.")?;
    writeln!(out, "    - Make sure all intended characters are present.")?;
    writeln!(out, "    - If you used backspace to edit your input, review carefully.\n")?;

    writeln!(out, "Is this correct? [Y/n]: ")?;
    out.flush()?;

    let mut response = String::new();
    io::stdin().read_line(&mut response)?;
    Ok(response)
}


/// Executes the given function (assumed to be costly) with a spinner displayed on screen.
pub fn with_spinner<F, T, E>(f: F) -> Result<T, E>
where
    F: FnOnce() -> Result<T, E>,
{
    let message = "Deriving secure BIP-39 passphrase from secret... This may take up to several minutes.";

    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::with_template("{spinner} {msg}").unwrap());
    pb.set_message(message.to_string());
    pb.enable_steady_tick(Duration::from_millis(120));

    let result = f();

    pb.finish_and_clear();

    result
}


/// Displays the given BIP-39 passphrase on screen for specified duration and clears the screen.
pub fn show_transient_output(passphrase: &str, display_duration: u64) -> Result<()> {
    let mut out = io::stdout();
    clear_terminal(&mut out)?;

    writeln!(out, "Your derived BIP-39 passphrase:\n\n{}\n", passphrase)?;
    writeln!(out, "You have {display_duration} seconds to copy it. Press Enter to clear early.")?;
    out.flush()?;

    wait_for_enter_or_timeout(display_duration)?;
    clear_terminal(&mut out)?;

    Ok(())
}


fn wait_for_enter_or_timeout(seconds: u64) -> Result<()> {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let _ = io::stdin().read_line(&mut String::new());
        let _ = tx.send(());
    });

    match rx.recv_timeout(Duration::from_secs(seconds)) {
        Ok(_) => Ok(()),//Enter key is pressed.
        Err(mpsc::RecvTimeoutError::Timeout) => Ok(()),//Timed out.
        Err(e) => Err(eyre!("{e:?}")),
    }
}


fn clear_terminal(out: &mut Stdout) -> Result<()> {
    out.execute(terminal::Clear(terminal::ClearType::All))?;
    out.execute(cursor::MoveTo(0, 0))?;
    Ok(())
}
