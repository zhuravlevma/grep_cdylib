use crate::{Functions, FunctionsFn};
use libloading::Library;
use std::sync::Arc;

#[derive(Clone)]
pub struct Lib {
    pub lib: Arc<Library>,
    pub functions: Functions,
}

impl Lib {
    pub unsafe fn new(lib: Library) -> Result<Self, anyhow::Error> {
        let load_fn: libloading::Symbol<FunctionsFn> = lib.get(b"functions")?;
        let functions = load_fn();

        if functions.size != std::mem::size_of::<Functions>() {
            return Err(anyhow::Error::msg(
                "Lib Functions size != app Functions size",
            ));
        }

        Ok(Self {
            lib: Arc::new(lib),
            functions,
        })
    }
}
