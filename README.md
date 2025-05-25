# sphragis

[![Latest Release](https://img.shields.io/github/v/release/aurynsinclair/sphragis)](https://github.com/aurynsinclair/sphragis/releases)
[![License](https://img.shields.io/github/license/aurynsinclair/sphragis)](./LICENSE)


**sphragis** is a command-line utility for secure, deterministic derivation of [BIP-39](https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki) passphrases using [Argon2id](https://github.com/p-h-c/phc-winner-argon2) KDF.  
It takes a user-defined secret phrase and configuration, ensuring consistent outputs and strong resistance against brute-force attacks.  

The name comes from the _sphragis_, an authorial “seal” in classical literature—symbolizing identity, authorship, and permanence.  
This tool brings that ancient metaphor into the cryptographic age: it seals your memory and forges a passphrase—your secure and reproducible credential for cryptographic identity.

If you are wondering why a BIP-39 passphrase matters—or why a passphrase should not be stored or remembered directly—  
see [Why use a BIP-39 passphrase — and why derive it from memory?](#why-use-a-bip-39-passphrase--and-why-derive-it-from-memory).


---

## ✨ Features

- 🔐 Derive secure, deterministic BIP-39 passphrases from a user-provided secret phrase
- 🧂 Configure Argon2id parameters and salt via JSON5 files
- 🖥️ Dual-mode operation: interactive prompts for human-friendly use, or standard input/output for scripting and automation
- 📤 Export salts as needed via the `generate-salt` subcommand


## 🚀 Quick Start

### 1. Prepare your configuration file (e.g., `sphragis.json5`):

```json5
{
  // Argon2 version
  "version": "0x13",

  // Argon2id parameters
  "params": {
    "m_cost": 1048576,     // 1 GiB memory
    "t_cost": 12,          // 12 iterations
    "p_cost": 1            // use 1 thread to prevent parallel GPU speedups
  },

  // Base64-encoded salt string (MUST be unique per user/device)
  "salt": "QyEVfkkKNre8LFhsc8qQSA=="
}
```

To ensure uniqueness and randomness, generate a 16-byte salt using:
```shell
sphragis generate-salt --length 16
```
Note: `--length 16` is the default and can be omitted.

For detailed explanations and recommended values for each setting,  
see the inline comments in [./sphragis.json5](./sphragis.json5).

### 2. Run the derivation:

```shell
sphragis derive --config sphragis.json5 --display-duration 60
```
Note: All arguments shown above are optional—`--config`, `--display-duration`, and even the subcommand `derive` default to the values shown.

You will be prompted to enter your secret phrase (it will be visible as you type).  
After confirmation, your input will be cleared from the screen for security.
The derived BIP-39 passphrase will be displayed briefly, then automatically hidden after the duration specified by `--display-duration`.

### ⚙️ Non-interactive & verbose modes

For scripting, debugging, or parameter tuning, `sphragis` also supports non-interactive operation via standard input/output and a `--verbose` mode for full output visibility.  
These features are documented [here](./docs/advanced-usage.md).

⚠️ **Caution:** While useful for automation and testing, these modes are **not recommended for actual passphrase derivation**, as sensitive input or output may be recorded in shell history, process logs, or CI environments.


## 📦 Releases

You can download prebuilt binaries for **Linux**, **macOS**, and **Windows** from the [GitHub Releases page](https://github.com/aurynsinclair/sphragis/releases).

---

## 🔐 Why use a BIP-39 passphrase — and why derive it from memory?

### 📛 Misconceptions around the "25th word"

The so-called “25th word” is a common but deeply misleading nickname for the BIP-39 passphrase.  
Not only does it trivialize its role as an essential layer of protection,  
it also falsely suggests that the passphrase must be chosen from the same fixed 2048-word list used for mnemonic phrases.

In reality, a BIP-39 passphrase can be **any UTF-8 string** of your choosing.  
It is not bound by any wordlist, length limit, or linguistic constraint —  
and its flexibility makes it a powerful tool for individual security design.

Far from being a trivial extra password, the passphrase is a powerful and flexible layer of protection over your seed phrase.  
It deserves to be understood not as an add-on, but as an integral part of a personal security model.

### 🚧 Threats that multisig can't defend against

Securing a BIP-39 seed phrase (12 or 24 words) in physical form inevitably makes it susceptible to theft, seizure, or coercion.  
Multisignature setups may mitigate some of these risks —  
but they do not protect against **state-level threats** such as border searches, device confiscation, or compelled disclosure.  
When traveling under regimes where fiat currency has collapsed or capital controls are aggressive,  
even robust multisig arrangements may prove insufficient.

In such scenarios, a **passphrase known only to memory** becomes the final layer of sovereignty —  
an identity no authority can seize, a key that exists only when you choose to recall it.  
It may even offer a form of [plausible deniability](https://medium.com/airgap-it/securing-your-crypto-with-plausible-deniability-and-bip-39-passphrases-3bb80be72e75),
should you ever be compelled to reveal your secrets.

### 🧠 The limits of human memory

But **memory is not entropy**.

Human minds are optimized for pattern and meaning, not randomness.  
Any phrase memorable enough to recall without fear of forgetting is, by definition, constrained by structure.  
That structure is vulnerable to targeted guessing and profiling attacks—especially when seeded with public information or linguistic patterns.

It is worth noting that attacks on crypto wallets are often economically motivated and highly automated.  
Even modest individual holdings can be viable targets, since attackers operate at scale and optimize for expected return.  
**What matters is not how much you own, but how easily you can be guessed.**

### 🛡️ Deriving strength through Argon2id

To defend against this, we apply a deliberately slow, memory-intensive key derivation function — **Argon2id**.  
Unlike traditional hashing algorithms, Argon2id is designed to consume significant RAM during computation,  
making large-scale brute-force attacks expensive, especially on parallel hardware like GPUs and ASICs.

This deliberate slowing-down of the derivation process transforms a human-friendly phrase into a cryptographically hardened credential,  
aligning the needs of memory with the demands of modern adversaries.


## 🚪 Applying sphragis Securely in Practice

- **Crafting a memorable yet strong secret phrase:**  
  Tips and examples for generating human-memorable phrases with meaningful structure—such as acrostics, semantic cues, or story-based techniques.  
  _(Work-in-progress)_

- **Compatibility and usage of BIP-39 passphrases across wallets:**  
  An overview of how popular hardware and software wallets support passphrases in practice—  
  including limitations, UI ergonomics, and potential security trade-offs.  
  _(Work-in-progress)_

---

## ⚠️ Security Notice

This is a security-sensitive tool. **Use at your own risk.**  
Make sure you fully understand the implications of passphrase derivation, as well as the secure handling of your configuration and seed data.

While this tool does **not** log or store your secret phrase,  
terminal sessions may be captured by the operating system, remote services, or monitoring tools.  
**Avoid running it on shared, untrusted, or monitored environments** — such as CI pipelines, remote servers, or containerized shells.

For maximum security, run `sphragis` on a private, local machine in a physically secure environment.


## 📜 License
MIT

---

Thank you for reading. Stay safe, and may your passphrases remain both memorable and secure.
