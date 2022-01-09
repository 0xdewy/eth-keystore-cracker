# Ethereum Keystore Cracker
Decrypt Ethereum keystores.

To generate password file check out: [cracken](https://github.com/shmuelamar/cracken)

## Usage
```
 USAGE:
    keystore_cracker [FLAGS] [OPTIONS] <keystore-directory> <output-directory> <passwords-file>

FLAGS:
    -d, --debug      Activate debug mode
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -p, --password <password>    New password for keystore (default: insecure)

ARGS:
    <keystore-directory>    Directory of keystore(s) to decrypt
    <output-directory>      Where new keystores will be saved
    <passwords-file>        Text file with password on each newline
      
```

## Features
[x] Decrypt keystore(s) from a list of potential passwords

[ ] [keepass](https://github.com/sseemayer/keepass-rs) integration 