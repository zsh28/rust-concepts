# Challenge 2 - Persistent Todo Queue CLI (Borsh)

This package implements a CLI todo app with a custom generic FIFO queue and Borsh-based disk persistence.

## What this package demonstrates

- Building a custom generic queue without external queue packages.
- FIFO task processing (`add`, `list`, `done`).
- Persisting queue state to disk with Borsh only.
- Restoring queue state on restart.
- Keeping queue and app logic modular and testable.

## File layout

- `src/lib.rs`
  - Public module wiring and exports.
- `src/todo.rs`
  - `Todo` data model.
- `src/queue.rs`
  - Generic `Queue<T>` implementation (`enqueue`, `dequeue`, `peek`, `len`, `is_empty`).
- `src/app.rs`
  - `TodoApp` service: load, persist, add, list, complete.
- `src/main.rs`
  - CLI entry point and command parsing.
- `src/tui.rs`
  - Ratatui interactive interface and keyboard event loop.
- `tests/queue_tests.rs`
  - FIFO behavior tests for the queue.
- `tests/app_tests.rs`
  - Persistence test across simulated restart.

## How data persistence works

1. On startup, `TodoApp::load_or_new` reads `todos.bin` (if present).
2. Bytes are deserialized using Borsh into `Vec<Todo>`.
3. Todos are loaded into the in-memory queue in FIFO order.
4. After every `add` or `done`, the queue is serialized back to `todos.bin`.

## Run tests for this package

```bash
cargo test -p challenge2-todo
```

## Run the CLI

```bash
cargo run -p challenge2-todo --bin todo
cargo run -p challenge2-todo --bin todo -- interactive
cargo run -p challenge2-todo --bin todo -- add "Buy groceries"
cargo run -p challenge2-todo --bin todo -- list
cargo run -p challenge2-todo --bin todo -- done
```

By default, this writes `todos.bin` in your current working directory.

When run with no command (or with `interactive`), the app opens a ratatui interface.

Ratatui controls:

- `a` add task (input mode)
- `Enter` submit task in input mode
- `Esc` cancel input mode
- `d` complete next task (FIFO)
- `x` or `Delete` delete currently selected task
- `j` / `k` or arrow keys to move highlight
- `q` quit
