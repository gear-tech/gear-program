// Copyright (C) 2021-2022 Gear Technologies Inc.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

//! Metadata parser
#![allow(dead_code)]
#![allow(unused_imports)]

mod env;
mod executor;
mod ext;
mod result;

/// Data used for the wasm exectuon.
pub type StoreData = ext::Ext;

/// Gear program metadata
///
/// See https://github.com/gear-tech/gear/blob/master/gstd/src/macros/metadata.rs.
const META_TYPES: [&str; 12] = [
    "meta_title",
    "meta_init_input",
    "meta_init_output",
    "meta_async_init_input",
    "meta_async_init_output",
    "meta_handle_input",
    "meta_handle_output",
    "meta_async_handle_input",
    "meta_async_handle_output",
    "meta_state_input",
    "meta_state_output",
    "meta_registry",
];

/// Gear program metadata
///
/// See https://github.com/gear-tech/gear/blob/master/gstd/src/macros/metadata.rs.
pub struct Metadata {
    pub meta_title: String,
    pub meta_init_input: String,
    pub meta_init_output: String,
    pub meta_async_init_input: String,
    pub meta_async_init_output: String,
    pub meta_handle_input: String,
    pub meta_handle_output: String,
    pub meta_async_handle_input: String,
    pub meta_async_handle_output: String,
    pub meta_state_input: String,
    pub meta_state_output: String,
    pub meta_registry: String,
}
