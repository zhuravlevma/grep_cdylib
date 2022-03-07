use crate::run::run_c;
use crate::search::{search_func, search_with_insensitive};
use crate::Config;
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
    search_case_insensitive: SearchWithCaseInsensitive,
    run: Run,
}
impl Default for FunctionsBlock {
    fn default() -> Self {
        Self {
            size: std::mem::size_of::<Self>(),
            search_string: search_func,
            search_case_insensitive: search_with_insensitive,
            run: run_c,
        }
    }
}

#[repr(u8)]
pub enum GetStrResult {
    Ok = 0,
    BufferTooSmall = 1,
    End = 3,
}

type SearchString = unsafe extern "C" fn(
    *mut u8,
    size: *mut usize,
    num: usize,
    *const c_char,
    *const c_char,
) -> GetStrResult;

type SearchWithCaseInsensitive = unsafe extern "C" fn(
    *mut u8,
    size: *mut usize,
    num: usize,
    *const c_char,
    *const c_char,
) -> GetStrResult;

type Run = unsafe extern "C" fn(*const Config) -> ();
