use std::error::Error;
use std::{fs, mem};
use std::env;
use std::os::raw::{c_char};
use std::ffi::{CStr, CString};
use std::slice;

#[no_mangle]
pub extern "C" fn get_my_integer() -> i32 {
    45
}
#[no_mangle]
pub unsafe extern "C" fn search_func(buffer: *mut u8, size: *mut usize, query: *const c_char, content: *const c_char) -> GetStrResult {
    let c_str_query = CStr::from_ptr(query);
    let c_str_content = CStr::from_ptr(content);
    let query_str = c_str_query.to_str().unwrap();
    let content_str = c_str_content.to_str().unwrap();
    // println!("{}", size as usize);
    let res = search(query_str, content_str);
    // let bytes = res.as_bytes();
    let required_size: usize = res.iter().map(|el| el.to_string().as_bytes().len()).sum();
    if *size < required_size {
        *size = required_size;
        return GetStrResult::BufferTooSmall;
    }
    // let mut str_vec = vec![];
    // res.iter().for_each(|el| {
    //     let length = el.as_bytes().len();
    //     let slice = *slice::from_raw_parts_mut(buffer as *mut u8, length);
    //     str_vec.push(slice)
    // });
    // let slice = slice::from_raw_parts_mut(buffer as *mut u8, required_size);
    // slice.copy_from_slice(&str_vec[1..]);
    println!("{:?}", res);
    GetStrResult::Ok
}


#[repr(C)]
pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
}


type GetInteger = unsafe extern "C" fn() -> i32;
type SearchString = unsafe extern "C" fn(*mut u8, size: *mut usize, *const c_char, *const c_char) -> GetStrResult;
#[allow(unused)]
#[repr(C)]
pub struct FunctionsBlock {
    size: usize,
    get_integer: GetInteger,
    search_string: SearchString,
}

#[repr(u8)]
pub enum GetStrResult {
    Ok = 0,
    BufferTooSmall = 1,
}


impl Default for FunctionsBlock {
    fn default() -> Self {
        Self {
            size: std::mem::size_of::<Self>(),
            get_integer: get_my_integer,
            search_string: search_func
        }
    }
}

#[no_mangle]
pub extern "C" fn functions() -> FunctionsBlock {
    FunctionsBlock::default()
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next(); // skip program name

        let query = match args.next() {
            None => return Err("query not received"),
            Some(value) => value
        };
        let filename = match args.next() {
            None => return Err("filename not received"),
            Some(value) => value
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {query, filename, case_sensitive })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|line| line.contains(query)).collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents.lines().filter(|line| line.to_lowercase().contains(&query)).collect()
}


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &content)
    } else {
        search_case_insensitive(&config.query, &content)
    };
    for line in results {
        println!("{:?}", line);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, productive, fast
Pick three.
        ";
        assert_eq!(
            vec!["safe, productive, fast"],
            search(query, contents)
        )
    }
    #[test]
    fn case_insensitive() {
        let query = "RuST";
        let contents = "\
Rust:
safe, productive, fast
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        )
    }
}
// impl<'a> TryFrom<&'a Content> for &'a String {
//     type Error = ImageError;
//
//     fn try_from(value: &'a RawPath) -> Result<Self, Self::Error> {
//         if value.0.is_null() {
//             return Err(ImageError::Parameter);
//         }
//
//         let s = unsafe { CStr::from_ptr(value.0) };
//         let utf8_str = match s.to_str() {
//             Ok(s) => s,
//             Err(_) => return Err(ImageError::Parameter),
//         };
//
//         let path: &Path = Path::new(utf8_str);
//         Ok(path)
//     }
// }