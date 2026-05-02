/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
use std::error::Error;
use std::io;

use qubit_error::error::BoxError;

fn accept_static_error(error: BoxError) -> Box<dyn Error + Send + Sync + 'static> {
    error
}

#[test]
fn test_box_error_is_send_sync_static_error_object() {
    let error: BoxError = Box::new(io::Error::other("boxed error"));
    let error = accept_static_error(error);

    assert_eq!(error.to_string(), "boxed error");
}
