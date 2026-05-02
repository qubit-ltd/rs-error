/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
use std::io;

use qubit_error::error::DynError;

fn error_message(error: &DynError) -> String {
    error.to_string()
}

#[test]
fn test_dyn_error_names_shared_error_object_bounds() {
    let error = io::Error::other("dynamic error");

    assert_eq!(error_message(&error), "dynamic error");
}
