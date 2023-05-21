# Ethereum Keystore Cracker

Multithreaded Ethereum keystore search and decrypt

To generate password file check out: 

[cracken](https://github.com/shmuelamar/cracken)

or

[pwfuzz-rs](https://github.com/mttaggart/pwfuzz-rs)

## Usage
```
Ethereum Keystore Cracker

USAGE:
    keystore_cracker [OPTIONS] <keystore-directory> <output-directory> <passwords-file>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -p, --password <password>    New password for keystore (default: insecure)

ARGS:
    <keystore-directory>    Directory of keystore(s) to decrypt
    <output-directory>      Where new keystores will be saved
    <passwords-file>        Text file with password on each newline

```