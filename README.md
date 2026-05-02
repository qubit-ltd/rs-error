# Qubit Error

Shared error handling helpers for Qubit Rust projects.

`qubit-error` currently provides boxed error aliases and explicit conversion
helpers for places where the concrete error type is intentionally erased.

## Features

- `DynError`: shared dynamic error trait-object bounds.
- `BoxError`: owned boxed dynamic error.
- `BoxResult<T>`: result alias using `BoxError`.
- `IntoBoxError`: explicit conversion helper for `map_err` and manual error
  construction.

## Installation

```toml
[dependencies]
qubit-error = "0.1.0"
```

## Quick Start

```rust
use qubit_error::{BoxResult, IntoBoxError};

fn parse_port(text: &str) -> BoxResult<u16> {
    text.parse::<u16>()
        .map_err(|error| error.into_box_error())
}
```

## Module Layout

- `error`: boxed error aliases and conversion helpers.

Top-level re-exports are provided for all current public items.
