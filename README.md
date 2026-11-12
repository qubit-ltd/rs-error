# Qubit Error

[![Rust CI](https://github.com/qubit-ltd/rs-error/actions/workflows/ci.yml/badge.svg)](https://github.com/qubit-ltd/rs-error/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://qubit-ltd.github.io/rs-error/coverage-badge.json)](https://qubit-ltd.github.io/rs-error/coverage/)
[![Crates.io](https://img.shields.io/crates/v/qubit-error.svg?color=blue)](https://crates.io/crates/qubit-error)
[![Rust](https://img.shields.io/badge/rust-1.94+-blue.svg?logo=rust)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![中文文档](https://img.shields.io/badge/文档-中文版-blue.svg)](README.zh_CN.md)

Shared error handling utilities for Rust.

## Overview

Qubit Error provides small, reusable error handling primitives for places where
the exact concrete error type is intentionally erased. It currently focuses on
boxed error aliases and an explicit conversion trait, and can grow with other
common error handling utilities over time.

## Design Goals

- **Tiny dependency surface**: provide standard-library-only utilities.
- **Explicit type erasure**: make conversion into boxed errors visible at call sites.
- **Integration friendly**: support callbacks, executable entry points, and stored source errors.
- **Concrete errors first**: encourage domain crates to keep structured public error types.
- **Future growth**: leave room for additional generic error utilities without coupling to domain crates.

## Features

### Boxed Error Aliases

- **`DynError`**: shared `dyn Error + Send + Sync + 'static` object bounds.
- **`BoxError`**: owned boxed dynamic error.
- **`BoxResult<T>`**: result alias using `BoxError`.

### Explicit Conversion

- **`IntoBoxError`**: converts concrete standard errors into `BoxError`.
- Useful with `map_err`, generic callback boundaries, and manual `Err` construction.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
qubit-error = "0.2"
```

## Quick Start

```rust
use qubit_error::{
    BoxResult,
    IntoBoxError,
};

fn parse_port(text: &str) -> BoxResult<u16> {
    text.parse::<u16>()
        .map_err(|error| error.into_box_error())
}

let port = parse_port("8080").expect("valid port should parse");
assert_eq!(port, 8080);
assert!(parse_port("not-a-port").is_err());
```

## When to Use Boxed Errors

Use boxed error utilities when callers only need to propagate, log, or store the
original source error:

- Small binaries or examples.
- Generic callback return types.
- Integration glue between otherwise unrelated error types.
- Source errors stored inside a higher-level concrete domain error.

Prefer concrete error enums or structs for public APIs where callers need
structured recovery.

## API Reference

- [`DynError`](https://docs.rs/qubit-error/latest/qubit_error/type.DynError.html) - shared dynamic error object bounds.
- [`BoxError`](https://docs.rs/qubit-error/latest/qubit_error/type.BoxError.html) - owned boxed dynamic error.
- [`BoxResult`](https://docs.rs/qubit-error/latest/qubit_error/type.BoxResult.html) - result alias using `BoxError`.
- [`IntoBoxError`](https://docs.rs/qubit-error/latest/qubit_error/trait.IntoBoxError.html) - explicit conversion trait.

## Dependencies

This crate has no runtime dependencies outside the Rust standard library.

## Coverage

Tests focus on type bounds, conversion behavior, and doctest examples. For
coverage report instructions, see [COVERAGE.md](COVERAGE.md).

## Related Projects

More Rust libraries from Qubit are published under the [qubit-ltd](https://github.com/qubit-ltd) organization on GitHub.

## Testing

```bash
# Run tests with the default feature set
cargo test

# Run tests with all declared features
cargo test --all-features

# Project CI checks
./.infra/bin/ci-check.sh

# Check code coverage
./.infra/bin/coverage.sh
```

## License

Copyright (c) 2025 - 2026. Haixing Hu. All rights reserved.

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE) for the
full license text.

## Contributing

Contributions are welcome. Please follow the Rust API guidelines, keep public
API documentation and tests current, and run `./.infra/bin/align-ci.sh` to format code and
`./.infra/bin/ci-check.sh` to satisfy CI requirements before submitting a pull request.

## Author

**Haixing Hu** - *Qubit Co. Ltd.*

Repository: [https://github.com/qubit-ltd/rs-error](https://github.com/qubit-ltd/rs-error)
