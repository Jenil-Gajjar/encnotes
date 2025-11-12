# encnotes

A command-line encrypted notes manager built with Rust.  
This project was created for practice and demonstrates modular CLI design, encryption, and secure file handling in Rust.

---

## Features

- End-to-end encryption using Argon2 + XChaCha20-Poly1305
- Master password–protected vault (`vault.enc`)
- Add, list, view, update, and delete notes
- Change master password securely
- Session-based login and logout
- JSON-based encrypted storage

---

## Commands

```
init             # Initialize a new encrypted vault
login            # Cache master password for this session
logout           # Clear cached session password
add              # Add a new note
list             # List all notes
view <query>     # View a note by ID or title
update           # Edit a note by ID
delete <id>      # Delete a note by ID
change-pwd       # Change master password
```

---

## Example

```
$ encnotes init
Enter Master Password:
Vault initialized successfully.

$ encnotes add
Enter Master Password:
Title: Example note
Description: This is an encrypted note.
Note saved successfully.

$ encnotes list
1. Example note [e3b1a09c-...]
```

---

## Dependencies

- clap
- serde, serde_json
- chrono
- uuid
- argon2
- chacha20poly1305
- base64
- rpassword
- rand
- anyhow

---

## Notes

- Vault file: `vault.enc`
- Session file: `/tmp/encnotes.session`
- Changing your master password re-encrypts the vault and saves a backup as `vault.enc.bak`.

---

## Purpose

This project was built for learning Rust concepts like modular design, encryption, and command-line tooling.  
It’s not intended for production use but serves as a solid reference or learning exercise.
