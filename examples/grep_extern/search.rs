use crate::interface::Config;
use crate::{GetStrResult, Lib};
use std::ffi::CString;

pub struct Search;

impl Search {
    pub fn search(lib: Lib, query: &str, content: &str) -> Vec<String> {
        let query = CString::new(query.as_bytes()).unwrap();
        let content = CString::new(content.as_bytes()).unwrap();
        let mut buf = Vec::new();
        let mut size = buf.len();
        let mut count = 0;
        let mut res = vec![];
        loop {
            let result = unsafe {
                (lib.functions.search_string)(
                    buf.as_mut_ptr(),
                    &mut size,
                    count,
                    query.as_ptr(),
                    content.as_ptr(),
                )
            };
            match result {
                GetStrResult::Ok => {
                    let c_str = String::from_utf8(buf).unwrap();
                    res.push(c_str);
                    buf = Vec::new();
                    size = buf.len();
                }
                GetStrResult::BufferTooSmall => {
                    buf.resize(size, 0);
                    unsafe {
                        (lib.functions.search_string)(
                            buf.as_mut_ptr(),
                            &mut size,
                            count,
                            query.as_ptr(),
                            content.as_ptr(),
                        )
                    };
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

    pub fn search_case_insensitive(lib: Lib, query: &str, content: &str) -> Vec<String> {
        let query = CString::new(query.as_bytes()).unwrap();
        let content = CString::new(content.as_bytes()).unwrap();
        let mut buf = Vec::new();
        let mut size = buf.len();
        let mut count = 0;
        let mut res = vec![];
        loop {
            let result = unsafe {
                (lib.functions.search_case_insensitive)(
                    buf.as_mut_ptr(),
                    &mut size,
                    count,
                    query.as_ptr(),
                    content.as_ptr(),
                )
            };
            match result {
                GetStrResult::Ok => {
                    let c_str = String::from_utf8(buf).unwrap();
                    res.push(c_str);
                    buf = Vec::new();
                    size = buf.len();
                }
                GetStrResult::BufferTooSmall => {
                    buf.resize(size, 0);
                    unsafe {
                        (lib.functions.search_case_insensitive)(
                            buf.as_mut_ptr(),
                            &mut size,
                            count,
                            query.as_ptr(),
                            content.as_ptr(),
                        )
                    };
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

    pub fn run(lib: Lib, config: Config) {
        let c = &config as *const Config;
        (lib.functions.run)(c)
    }
}
