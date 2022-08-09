#![cfg(test)]

use crate::metadata::{registry::LocalRegistry, Metadata};
use parity_scale_codec::{Decode, Encode};
use scale_info::{
    form::Form, IntoPortable, MetaType, PortableRegistry, Registry, TypeDef, TypeInfo,
};

#[derive(TypeInfo)]
pub struct Composite {
    string: String,
    number: u32,
    array: [u8; 32],
}

#[test]
fn test_parsing_metadata() {
    let demo_meta = include_bytes!("../../res/demo_meta.meta.wasm");
    let metadata = Metadata::of(demo_meta).expect("get metadata failed");

    assert_eq!(
        metadata,
        Metadata {
            meta_title: Some("Example program with metadata".into()),
            meta_init_input: Some("MessageInitIn".into()),
            meta_init_output: Some("MessageInitOut".into()),
            meta_async_init_input: Some("MessageInitAsyncIn".into()),
            meta_async_init_output: Some("MessageInitAsyncOut".into()),
            meta_handle_input: Some("MessageIn".into()),
            meta_handle_output: Some("MessageOut".into()),
            meta_async_handle_input: Some("MessageHandleAsyncIn".into()),
            meta_async_handle_output: Some("MessageHandleAsyncOut".into()),
            meta_state_input: Some("Option<Id>".into()),
            meta_state_output: Some("Vec<Wallet>".into()),
            meta_registry: None
        }
    );

    println!(
        "{:#}",
        metadata,
        // format!("{:#}", metadata)
        //     .replace('"', "")
        //     .replace("\\n", "\n")
        // .replace('"', "")
        // .replace("\n", &format!("\n{:>4}", ' '))
    );
}
