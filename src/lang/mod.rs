/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Error Language Layer
//!
//! Provides shared error handling helpers.
//!
//! # Author
//!
//! Haixing Hu

pub mod error;

pub use error::{
    BoxError,
    BoxResult,
    DynError,
    IntoBoxError,
};
