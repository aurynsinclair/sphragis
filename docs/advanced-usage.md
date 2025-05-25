# Advanced Usage

## Non-interactive mode

You can run `sphragis` in non-interactive mode by piping the secret phrase into standard input, for example:

```sh
echo <YOUR SECRET PHRASE> | sphragis
```

This will execute the derive command with the provided input and print the resulting BIP-39 passphrase to standard output.

⚠️ **Caution:**  
Non-interactive mode is intended for testing, debugging, and parameter tuning only.  
When used with real secrets or actual passphrases, it may expose sensitive information via command history, shell logs, process monitoring tools, or captured stdout.  

**Use with extreme care, and only if you fully understand the risks.**


## `--verbose` flag

The --verbose flag is intended specifically for non-interactive usage,
where full visibility into the derivation process and output is required (e.g., debugging or automation).

```sh
echo <YOUR SECRET PHRASE> | sphragis --verbose
```
