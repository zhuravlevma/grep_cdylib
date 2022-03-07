use crate::search::search_func;
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn functions() -> FunctionsBlock {
    FunctionsBlock::default()
}

#[allow(unused)]
#[repr(C)]
pub struct FunctionsBlock {
    size: usize,
    search_string: SearchString,
}
type SearchString = unsafe extern "C" fn(
    *mut u8,
    size: *mut usize,
    num: usize,
    *const c_char,
    *const c_char,
) -> GetStrResult;
#[repr(u8)]
pub enum GetStrResult {
    Ok = 0,
    BufferTooSmall = 1,
    End = 3,
}

impl Default for FunctionsBlock {
    fn default() -> Self {
        Self {
            size: std::mem::size_of::<Self>(),
            search_string: search_func,
        }
    }
}
