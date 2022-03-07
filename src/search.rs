use crate::GetStrResult;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::slice;

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn search_func(
    buffer: *mut u8,
    size: *mut usize,
    num: usize,
    query: *const c_char,
    content: *const c_char,
) -> GetStrResult {
    let c_str_query = CStr::from_ptr(query);
    let c_str_content = CStr::from_ptr(content);
    let query_str = c_str_query.to_str().unwrap();
    let content_str = c_str_content.to_str().unwrap();
    let res = search(query_str, content_str);
    send_array_with_strings(res, num, size, buffer)
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn search_with_insensitive(
    buffer: *mut u8,
    size: *mut usize,
    num: usize,
    query: *const c_char,
    content: *const c_char,
) -> GetStrResult {
    let c_str_query = CStr::from_ptr(query);
    let c_str_content = CStr::from_ptr(content);
    let query_str = c_str_query.to_str().unwrap();
    let content_str = c_str_content.to_str().unwrap();
    let res = search_case_insensitive(query_str, content_str);
    send_array_with_strings(res, num, size, buffer)
}

unsafe fn send_array_with_strings(
    data: Vec<&str>,
    index: usize,
    size: *mut usize,
    buffer: *mut u8,
) -> GetStrResult {
    let elem = data.get(index);
    return match elem {
        None => GetStrResult::End,
        Some(elem) => {
            let s = elem.to_string();
            let bytes = s.as_bytes();
            let required_size = bytes.len();
            if *size < required_size {
                *size = required_size;
                return GetStrResult::BufferTooSmall;
            }
            let slice = slice::from_raw_parts_mut(buffer as *mut u8, required_size);
            slice.copy_from_slice(bytes);

            GetStrResult::Ok
        }
    };
}
