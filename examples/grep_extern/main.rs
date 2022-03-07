mod interface;

use std::ffi::{CString};
use std::sync::Arc;
use libloading::Library;
use interface::{lib_path, FunctionsFn, Functions};
use crate::interface::GetStrResult;

#[derive(Clone)]
struct Lib {
    lib: Arc<Library>,
    functions: Functions,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lib = unsafe {
        let lib = libloading::Library::new(lib_path())?;
        Lib::new(lib)?
    };
    let num = unsafe { lib.get_integer() };
    let result = unsafe { lib.search_string("Ha", "Hahaha\n HjHo\n HoHaha\n ") };
    println!("{}", num);
    Ok(())
}

pub struct GrepFactory {
    lib: Lib,
}

impl GrepFactory {
    pub fn new() -> Result<Self, anyhow::Error> {
        let lib = unsafe {
            let lib = libloading::Library::new(lib_path())?;
            Lib::new(lib)?
        };

        Ok(Self { lib })
    }
    //
    // pub fn get_grep() -> Result<Image, anyhow::Error> {
    //
    // }
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

    pub unsafe fn get_integer(&self) -> i32 {
        (self.functions.get_integer)()
    }

    pub unsafe fn search_string(&self, query: &str, content: &str) -> () {
        let query =  CString::new(query.as_bytes()).unwrap();
        let content = CString::new(content.as_bytes()).unwrap();
        let mut buf = Vec::new();
        let mut size = buf.len();
        let mut result = (self.functions.search_string)(buf.as_mut_ptr(), &mut size, query.as_ptr(), content.as_ptr());
        if let GetStrResult::BufferTooSmall = result {
            buf.resize(size, 0);
            unsafe { (self.functions.search_string)(buf.as_mut_ptr(), &mut size, query.as_ptr(), content.as_ptr()) };
        }

    }
}