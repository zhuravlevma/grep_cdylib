use std::os::raw::c_char;
use std::path::Path;

pub fn lib_path() -> &'static Path {
    Path::new("target/release/libminigrep.dylib")
}

#[repr(u8)]
pub enum GetStrResult {
    Ok,
    BufferTooSmall,
    End,
}

#[repr(C)]
#[derive(Clone)]
pub struct Functions {
    pub size: usize,
    pub search_string: SearchString,
    pub search_case_insensitive: SearchWithCaseInsensitive,
    pub run: Run,
}

pub type FunctionsFn = unsafe extern "C" fn() -> Functions;
pub type SearchString = unsafe extern "C" fn(
    *mut u8,
    size: *mut usize,
    num: usize,
    *const c_char,
    *const c_char,
) -> GetStrResult;

pub type SearchWithCaseInsensitive = unsafe extern "C" fn(
    *mut u8,
    size: *mut usize,
    num: usize,
    *const c_char,
    *const c_char,
) -> GetStrResult;

#[repr(C)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

type Run = extern "C" fn(*const Config) -> ();
