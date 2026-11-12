// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::io;

use qubit_error::error::BoxResult;

#[derive(Debug)]
struct WrapperError {
    marker: u32,
    source: io::Error,
}

impl Display for WrapperError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "marker {}: {}", self.marker, self.source)
    }
}

impl Error for WrapperError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}

fn fail_with_question_mark() -> BoxResult<()> {
    Err(io::Error::other("box result failure"))?;
    Ok(())
}

#[test]
fn test_box_result_accepts_question_mark_conversion() {
    let error = fail_with_question_mark().expect_err("io error should be boxed");

    assert_eq!(error.to_string(), "box result failure");
}

fn fail_with_wrapped_error() -> BoxResult<()> {
    Err(WrapperError {
        marker: 23,
        source: io::Error::new(io::ErrorKind::NotFound, "missing"),
    })?;
    Ok(())
}

#[test]
fn test_question_mark_preserves_type_fields_and_source() {
    let error = fail_with_wrapped_error().expect_err("must fail");

    assert_eq!(error.downcast_ref::<WrapperError>().expect("original type").marker, 23);
    let source = error
        .source()
        .expect("source retained")
        .downcast_ref::<io::Error>()
        .expect("I/O source");
    assert_eq!(source.kind(), io::ErrorKind::NotFound);
    assert_eq!(source.to_string(), "missing");
}
