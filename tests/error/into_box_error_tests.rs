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

use qubit_error::error::BoxError;
use qubit_error::error::IntoBoxError;

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

fn convert_error() -> BoxError {
    io::Error::other("converted error").into_box_error()
}

#[test]
fn test_into_box_error_boxes_standard_error() {
    let error = convert_error();

    assert_eq!(error.to_string(), "converted error");
}

#[test]
fn test_into_box_error_preserves_type_fields_and_source() {
    let error = WrapperError {
        marker: 17,
        source: io::Error::new(io::ErrorKind::PermissionDenied, "denied"),
    }
    .into_box_error();

    assert_eq!(error.downcast_ref::<WrapperError>().expect("original type").marker, 17);
    let source = error
        .source()
        .expect("source retained")
        .downcast_ref::<io::Error>()
        .expect("I/O source");
    assert_eq!(source.kind(), io::ErrorKind::PermissionDenied);
    assert_eq!(source.to_string(), "denied");
}
