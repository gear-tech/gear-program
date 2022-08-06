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
    AsContext, AsContextMut, Engine, Extern, Func, Instance, Linker, Memory, Module, Store,
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
        let func = self.func(meta)?.to_raw(&self.store.as_context());
        let mem = memory.data(&self.store);

        println!("{:?}", func);
        println!("{:?}", mem.len());

        let ptr = mem[func..(func + 4)][0] as usize;
        println!("{:?}", ptr);

        let len = mem[(func + 4)..(func + 8)][0] as usize;

        Ok(String::from_utf8_lossy(&mem[ptr..len]).into())
    }
}

#[test]
fn test_parsing_metadata() {
    let demo_meta = include_bytes!("../../res/demo_meta.meta.wasm");
    execute(demo_meta, |mut reader| unsafe {
        let memory = reader.memory().expect("Memory not exists");
        reader
            .meta(&memory, "meta_title")
            .expect("Read metadata failed");

        Ok(())
    })
    .unwrap();
}
