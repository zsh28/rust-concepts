# Challenge 1 - Generic Storage with Multiple Serialization Formats

This package implements a generic `Storage<T, S>` system that can save/load values with different serializer implementations.

## What this package demonstrates

- Generic trait design with a serializer interface.
- Multiple serializer implementations behind one trait.
- `PhantomData<T>` for zero-cost type tracking in storage.
- Cross-library trait bounds for Borsh, Wincode, and JSON.
- Error handling with `Result` and a custom `StorageError`.

## File layout

- `src/lib.rs`
  - Public module wiring and exports.
- `src/error.rs`
  - `StorageError` for all storage/serialization failures.
- `src/compatible.rs`
  - `StorageCompatible` trait with all required bounds for supported formats.
- `src/serializer.rs`
  - `Serializer` trait with `to_bytes` and `from_bytes`.
- `src/formats.rs`
  - Format adapters: `Borsh`, `Wincode`, `Json`.
- `src/storage.rs`
  - Generic `Storage<T, S>` container implementation.
- `src/models.rs`
  - `Person` sample type used for tests.
- `tests/storage_tests.rs`
  - Round-trip tests for Borsh, Wincode, and JSON.

## How the pieces connect

1. `Storage<T, S>` stores raw bytes internally.
2. `S: Serializer` decides how bytes are produced/consumed.
3. `T: StorageCompatible` ensures the type works across all required formats.
4. `save` serializes `T -> Vec<u8>`, and `load` deserializes `Vec<u8> -> T`.

## Run tests for this package

```bash
cargo test -p challenge1-storage
```

## Optional quick usage snippet

```rust
use challenge1_storage::{Borsh, Person, Storage};

let person = Person {
    name: "Andre".to_string(),
    age: 30,
};

let mut storage = Storage::new(Borsh);
storage.save(&person)?;
let loaded: Person = storage.load()?;
assert_eq!(loaded.name, "Andre");
# Ok::<(), challenge1_storage::StorageError>(())
```
