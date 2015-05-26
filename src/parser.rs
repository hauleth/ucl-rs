use libucl_sys::{
    ucl_parser,
    ucl_parser_new,
    ucl_parser_free,
    ucl_parser_add_file,
    ucl_parser_add_chunk,
    ucl_parser_get_error,
    ucl_parser_get_object
};
use libc::size_t;

use super::utils;
use super::object::Object;

use std::path::Path;

pub struct Parser {
    parser: *mut ucl_parser,
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            parser: unsafe { ucl_parser_new(0x0) }
        }
    }

    pub fn parse<T: AsRef<str>>(mut self, string: T) -> Result<Object, String> {
        let len = string.as_ref().len() as size_t;
        let result = unsafe { ucl_parser_add_chunk(self.parser, utils::to_c_str(string), len) };

        if result {
            Ok(self.get_object().unwrap())
        } else {
            Err(self.get_error().unwrap())
        }
    }

    pub fn parse_file<T: AsRef<Path>>(mut self, path: T) -> Result<Object, String> {
        let filename = path.as_ref().to_str().unwrap();
        let result = unsafe { ucl_parser_add_file(self.parser, utils::to_c_str(filename)) };

        if result {
            Ok(self.get_object().unwrap())
        } else {
            Err(self.get_error().unwrap())
        }
    }

    fn get_object(&mut self) -> Option<Object> {
        Object::from_cptr(unsafe { ucl_parser_get_object(self.parser) })
    }

    fn get_error(&mut self) -> Option<String> {
        let err = unsafe { ucl_parser_get_error(self.parser) };

        utils::to_str(err)
    }
}

impl Drop for Parser {
    fn drop(&mut self) {
        unsafe { ucl_parser_free(self.parser) }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_string_parsing() {
        let p = Parser::new();
        let s = r#"lol = "lol""#;

        assert!(p.parse(s).is_ok());
    }

    #[test]
    fn test_empty_string_parsing() {
        let p = Parser::new();
        let s = r#""#;

        assert!(p.parse(s).is_ok());
    }

    #[test]
    fn test_key_fetching() {
        let p = Parser::new();
        let s = r#"lol = 10"#;
        let res = p.parse(s).unwrap();

        assert_eq!(res.fetch("lol").unwrap().as_int(), Some(10));
    }
}
