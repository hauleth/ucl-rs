use libucl_sys::*;
use libc::size_t;

use utils;
use object::{
    self,
    Object
};

use std::path::Path;

bitflags! {
    flags ParserFlags: i32 {
        const DEFAULT            = 0x0,
        const LOWERCASE          = 0x1,
        const ZEROCOPY           = 0x2,
        const NO_TIME            = 0x4,
        const NO_IMPLICIT_ARRAYS = 0x8
    }
}

pub struct Parser {
    parser: *mut ucl_parser,
}

impl Parser {
    /// Create new parser instance with default options
    pub fn new() -> Self {
        Self::with_flags(DEFAULT)
    }

    /// Create new parser with given option flags
    ///
    /// Flags:
    ///
    /// - `DEFAULT` - default configuration
    /// - `LOWERCASE` - convert all keys to lower case
    /// - `ZEROCOPY` - parse input in zero-copy mode if possible (you must ensure that input memory
    ///   is not freed if an object is in use)
    /// - `NO_TIME` - do not parse time and treat it's value as string
    /// - `NO_IMPLICIT_ARRAYS` - create explicit arrays instead of implicit ones
    ///
    /// # Examples
    ///
    /// ```rust
    /// let parser = ucl::Parser::with_flags(ucl::parser::LOWERCASE);
    /// let doc = parser.parse("A = b").unwrap();
    ///
    /// assert!(doc.fetch("a").is_some());
    /// ```
    pub fn with_flags(flags: ParserFlags) -> Self {
        Parser {
            parser: unsafe { ucl_parser_new(flags.bits()) }
        }
    }

    /// Parse given string. Returns root object on success.
    ///
    /// It moves out `Parser`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// assert!(ucl::Parser::new().parse("a = b").is_ok());
    /// assert!(ucl::Parser::new().parse("a =").is_err());
    /// ```
    pub fn parse<T: AsRef<str>>(mut self, string: T) -> Result<Object, String> {
        let len = string.as_ref().len() as size_t;
        let result = unsafe { ucl_parser_add_chunk(self.parser, utils::to_c_str(string), len) };

        if result {
            Ok(self.get_object().unwrap())
        } else {
            Err(self.get_error().unwrap())
        }
    }

    /// Parse file at given `Path`.
    ///
    /// It moves out `Parser`.
    pub fn parse_file<T: AsRef<Path>>(mut self, path: T) -> Result<Object, String> {
        let filename = path.as_ref().to_str().unwrap();
        let result = unsafe { ucl_parser_add_file(self.parser, utils::to_c_str(filename)) };

        if result {
            Ok(self.get_object().unwrap())
        } else {
            Err(self.get_error().unwrap())
        }
    }

    /// Register new variable
    ///
    /// # Examples
    ///
    /// ```rust
    /// let p = ucl::Parser::new();
    /// p.register_var("LOL".to_string(), "test".to_string());
    /// let res = p.parse("lol = $LOL").unwrap();
    ///
    /// assert_eq!(res.fetch("lol").unwrap().as_string(), Some("test".to_string()));
    /// ```
    pub fn register_var(&self, name: String, value: String) {
        unsafe {
            ucl_parser_register_variable(self.parser, utils::to_c_str(name), utils::to_c_str(value))
        }
    }

    fn get_object(&mut self) -> Option<Object> {
        object::Builder::from_ptr(unsafe { ucl_parser_get_object(self.parser) }).map(|o| o.build())
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
    fn string_parsing() {
        let p = Parser::new();
        let s = r#"lol = "lol""#;

        assert!(p.parse(s).is_ok());
    }

    #[test]
    fn empty_string_parsing() {
        let p = Parser::new();
        let s = r#""#;

        assert!(p.parse(s).is_ok());
    }

    #[test]
    fn key_fetching() {
        let p = Parser::new();
        let s = r#"lol = 10"#;
        let res = p.parse(s).unwrap();

        assert_eq!(res.fetch("lol").unwrap().as_int(), Some(10));
    }

    #[test]
    fn flags() {
        let s = r#"LoL = 10"#;
        let p = Parser::with_flags(DEFAULT);
        let res = p.parse(s).unwrap();

        assert!(res.fetch("lol").is_none());

        let p = Parser::with_flags(LOWERCASE);
        let res = p.parse(s).unwrap();

        assert_eq!(res.fetch("lol").unwrap().as_int(), Some(10));
    }

    #[test]
    fn variables() {
        let s = r#"lol = $LOL"#;
        let p = Parser::new();
        p.register_var("LOL".to_string(), "test".to_string());
        let res = p.parse(s).unwrap();

        assert_eq!(res.fetch("lol").unwrap().as_string(), Some("test".to_string()));
    }
}
