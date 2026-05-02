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

use qubit_error::error::BoxResult;

fn fail_with_question_mark() -> BoxResult<()> {
    Err(io::Error::other("box result failure"))?;
    Ok(())
}

#[test]
fn test_box_result_accepts_question_mark_conversion() {
    let error = fail_with_question_mark().expect_err("io error should be boxed");

    assert_eq!(error.to_string(), "box result failure");
}
