use std::os::raw::c_char;
use std::path::Path;

pub fn lib_path() -> &'static Path {
    Path::new("target/release/libminigrep.dylib")
}

#[repr(u8)]
pub enum GetStrResult {
    Ok = 0,
    BufferTooSmall = 1,
}


#[repr(C)]
#[derive(Clone)]
pub struct Functions {
    pub size: usize,
    pub get_integer: GetInteger,
    pub search_string: SearchString
}

pub type GetInteger = unsafe extern "C" fn() -> i32;
pub type FunctionsFn = unsafe extern "C" fn() -> Functions;
pub type SearchString = unsafe extern "C" fn(*mut u8, size: *mut usize, *const c_char, *const c_char) -> GetStrResult;