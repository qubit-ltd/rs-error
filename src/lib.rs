/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Qubit Error
//!
//! Provides shared error handling helpers for Rust applications.
//!
//! # Author
//!
//! Haixing Hu

/// Boxed error aliases and conversion helpers.
pub mod error;

pub use error::{
    BoxError,
    BoxResult,
    DynError,
    IntoBoxError,
};
