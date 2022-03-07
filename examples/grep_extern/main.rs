mod interface;

use interface::GetStrResult;
use interface::{lib_path, Functions, FunctionsFn};
use libloading::Library;
use std::ffi::CString;
use std::sync::Arc;

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
    let result = unsafe { lib.search_string("Ha", "Hahaha\n HjHo\n HoHaha\n ") };
    println!("{:?}", result);
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

    pub unsafe fn search_string(&self, query: &str, content: &str) -> Vec<String> {
        let query = CString::new(query.as_bytes()).unwrap();
        let content = CString::new(content.as_bytes()).unwrap();
        let mut buf = Vec::new();
        let mut size = buf.len();
        let mut count = 0;
        let mut res = vec![];
        loop {
            let result = (self.functions.search_string)(
                buf.as_mut_ptr(),
                &mut size,
                count,
                query.as_ptr(),
                content.as_ptr(),
            );
            match result {
                GetStrResult::Ok => {
                    let c_str = String::from_utf8(buf).unwrap();
                    res.push(c_str);
                    buf = Vec::new();
                    size = buf.len();
                }
                GetStrResult::BufferTooSmall => {
                    buf.resize(size, 0);
                    (self.functions.search_string)(
                        buf.as_mut_ptr(),
                        &mut size,
                        count,
                        query.as_ptr(),
                        content.as_ptr(),
                    );
                    continue;
                }
                GetStrResult::End => {
                    break;
                }
            }
            count += 1;
        }
        res
    }
}
