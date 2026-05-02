/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! # Qubit Error
//!
//! Provides shared error handling helpers for Rust applications.
//!

/// Boxed error aliases and conversion helpers.
pub mod error;

pub use error::{
    BoxError,
    BoxResult,
    DynError,
    IntoBoxError,
};
