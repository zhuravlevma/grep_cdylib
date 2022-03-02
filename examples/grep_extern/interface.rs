use std::os::raw::c_char;
use std::path::Path;

pub fn lib_path() -> &'static Path {
    Path::new("target/release/libminigrep.dylib")
}

#[repr(transparent)]
pub struct Query(pub *const c_char);

#[repr(transparent)]
pub struct Content(pub *const c_char);

type GetInteger = unsafe extern "C" fn() -> i32;
#[repr(C)]
#[derive(Clone)]
pub struct Functions {
    pub size: usize,
    pub get_integer: GetInteger,
    pub search_string: SearchString
}
pub type FunctionsFn = unsafe extern "C" fn() -> Functions;
pub type SearchString = for<'a> unsafe extern "C" fn(&'a Query, &'a Content) -> *const *const c_char;