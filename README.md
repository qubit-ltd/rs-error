# Qubit Error

[![Rust CI](https://github.com/qubit-ltd/rs-error/actions/workflows/ci.yml/badge.svg)](https://github.com/qubit-ltd/rs-error/actions/workflows/ci.yml)
[![Coverage Status](https://coveralls.io/repos/github/qubit-ltd/rs-error/badge.svg?branch=main)](https://coveralls.io/github/qubit-ltd/rs-error?branch=main)
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
qubit-error = "0.2.0"
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

## Testing & Code Coverage

This project keeps tests focused on type bounds, conversion behavior, and
doctest examples.

### Running Tests

```bash
# Run all tests
cargo test

# Run with coverage report
./coverage.sh

# Generate text format report
./coverage.sh text

# Run CI checks (format, clippy, test, coverage, audit)
./ci-check.sh
```

### Coverage Metrics

See [COVERAGE.md](COVERAGE.md) for detailed coverage statistics.

## Dependencies

This crate has no runtime dependencies outside the Rust standard library.

## License

Copyright (c) 2025 - 2026. Haixing Hu, Qubit Co. Ltd. All rights reserved.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

See [LICENSE](LICENSE) for the full license text.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Development Guidelines

- Follow the Rust API guidelines.
- Keep utilities generic and independent from domain crates.
- Document all public APIs with examples when they clarify usage.
- Run `./ci-check.sh` before submitting PRs.

## Author

**Haixing Hu** - *Qubit Co. Ltd.*

## Related Projects

More Rust libraries from Qubit are published under the [qubit-ltd](https://github.com/qubit-ltd) organization on GitHub.

---

Repository: [https://github.com/qubit-ltd/rs-error](https://github.com/qubit-ltd/rs-error)
