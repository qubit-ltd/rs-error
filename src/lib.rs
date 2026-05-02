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

pub mod lang;

/// Boxed error aliases and conversion helpers.
pub mod error {
    pub use crate::lang::error::*;
}

pub use lang::error::{
    BoxError,
    BoxResult,
    DynError,
    IntoBoxError,
};
