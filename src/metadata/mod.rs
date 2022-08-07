// Copyright (C) 2021-2022 Gear Technologies Inc.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

//! Metadata parser
#![allow(dead_code)]
#![allow(unused_imports)]

mod env;
mod executor;
mod ext;
mod result;

pub use result::{Error, Result};

/// Data used for the wasm exectuon.
pub type StoreData = ext::Ext;

macro_rules! construct_metadata {
    ($($meta:ident),+) => {
        /// Gear program metadata
        ///
        /// See https://github.com/gear-tech/gear/blob/master/gstd/src/macros/metadata.rs.
        #[derive(Debug)]
        pub struct Metadata {
            $(
                pub $meta: Option<String>,
            )+
        }

        impl Metadata {
            /// Get metadata of "*meta.wasm"
            pub fn of(bin: &[u8]) -> Result<Self> {
                executor::execute(bin, |mut reader| -> Result<Self> {
                    let memory = reader.memory()?;

                    unsafe {
                        Ok(Self {
                            $(
                                $meta: reader.meta(&memory, stringify!($meta)).ok(),
                            )+
                        })
                    }
                })
            }
        }
    };
}

construct_metadata![
    meta_title,
    meta_init_input,
    meta_init_output,
    meta_async_init_input,
    meta_async_init_output,
    meta_handle_input,
    meta_handle_output,
    meta_async_handle_input,
    meta_async_handle_output,
    meta_state_input,
    meta_state_output,
    meta_registry
];

#[test]
fn test_parsing_metadata() {
    use parity_scale_codec::Decode;
    use scale_info::PortableRegistry;

    let demo_meta = include_bytes!("../../res/demo_meta.meta.wasm");
    let metadata = Metadata::of(demo_meta).expect("get metadata failed");
    let registry = PortableRegistry::decode(
        &mut hex::decode(metadata.meta_registry.unwrap())
            .unwrap()
            .as_ref(),
    )
    .unwrap();

    println!("{:#?}", registry.types());
}
