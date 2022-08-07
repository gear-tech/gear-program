// Copyright (C) 2021-2022 Gear Technologies Inc.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

//! WASM executor for getting metadata from `*.meta.wasm`
use crate::metadata::{
    env,
    ext::Ext,
    result::{Error, Result},
    StoreData,
};
use wasmtime::{
    AsContext, AsContextMut, Engine, Extern, Func, Instance, Linker, Memory, Module, Store, Val,
};

/// Exeucte wasm binary
pub fn execute<R>(wasm: &[u8], f: fn(Reader) -> Result<R>) -> Result<R> {
    let engine = Engine::default();
    let module = Module::new(&engine, &mut &wasm[..])?;
    let mut store = Store::new(&engine, Default::default());

    // 1. Construct linker.
    let mut linker = <Linker<StoreData>>::new(&engine);
    env::apply(&mut store, &mut linker)?;

    // 2. Construct instance.
    let instance = linker.instantiate(&mut store, &module)?;

    f(Reader {
        instance,
        linker,
        store,
    })
}

/// Reader for reading metadata declaration from "*.meta.wasm"
pub struct Reader {
    instance: Instance,
    linker: Linker<StoreData>,
    pub store: Store<StoreData>,
}

impl Reader {
    /// Get function from wasm instance
    pub fn func(&mut self, name: impl AsRef<str>) -> Result<Func> {
        let meta = name.as_ref();
        self.instance
            .get_func(self.store.as_context_mut(), meta)
            .ok_or(Error::MetadataNotExists(meta.into()))
    }

    /// Get memory from wasm instance
    pub fn memory(&mut self) -> Result<Memory> {
        if let Some(Extern::Memory(mem)) =
            self.linker
                .get(self.store.as_context_mut(), "env", "memory")
        {
            Ok(mem)
        } else {
            Err(Error::MemoryNotExists)
        }
    }

    /// Read metadata from meta type
    pub unsafe fn meta(&mut self, memory: &Memory, meta: &str) -> Result<String> {
        let mut res = [Val::null()];
        self.func(meta)?.call(&mut self.store, &[], &mut res)?;

        let at = if let Val::I32(at) = res[0] {
            at as usize
        } else {
            return Err(Error::ReadMetadataFailed(meta.into()));
        };

        let mem = memory.data(&self.store);

        let mut ptr_bytes = [0; 4];
        ptr_bytes.copy_from_slice(&mem[at..(at + 4)]);
        let ptr = i32::from_le_bytes(ptr_bytes) as usize;

        let mut len_bytes = [0; 4];
        len_bytes.copy_from_slice(&mem[(at + 4)..(at + 8)]);
        let len = i32::from_le_bytes(len_bytes) as usize;

        Ok(String::from_utf8_lossy(&mem[ptr..(ptr + len)]).into())
    }
}
