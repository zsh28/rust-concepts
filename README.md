# Rust Concepts Workspace

This repository is a Cargo workspace with two independent challenge packages.

## Workspace packages

- `challenge1-storage`
  - Generic storage system with three serialization formats: Borsh, Wincode, JSON.
- `challenge2-todo`
  - Persistent CLI todo queue using a custom FIFO queue and Borsh file storage.

## Repository structure

- `Cargo.toml`
  - Workspace definition.
- `challenge1-storage/`
  - Challenge 1 package (library + tests).
- `challenge2-todo/`
  - Challenge 2 package (library + CLI binary + tests).

## Run commands

Run all workspace tests:

```bash
cargo test
```

Run package tests independently:

```bash
cargo test -p challenge1-storage
cargo test -p challenge2-todo
```

Run todo CLI (challenge 2):

```bash
cargo run -p challenge2-todo --bin todo
cargo run -p challenge2-todo --bin todo -- interactive
cargo run -p challenge2-todo --bin todo -- add "Buy groceries"
cargo run -p challenge2-todo --bin todo -- list
cargo run -p challenge2-todo --bin todo -- done
```

## Learn more

Each package has its own walkthrough:

- `challenge1-storage/README.md`
- `challenge2-todo/README.md`
