//! # UCL (Universal Configuration Library)
//!
//! This library is parser for UCL files.
//!
//! ## Basic structure
//!
//! UCL provide support for 2 different syntaxes:
//!
//! - JSON
//!
//!     ```javascript
//!     {
//!         "param": "value",
//!         "section": {
//!             "flag": true,
//!             "number": 10000,
//!             "subsection": {
//!                 "hosts": [
//!                     {
//!                         "host": "localhost",
//!                         "port": 9000
//!                     },
//!                     {
//!                         "host": "remotehost",
//!                         "port": 9090
//!                     }
//!             }
//!         }
//!     }
//!     ```
//!
//! - nginx like UCL
//!
//!     ```nginx
//!     param = value;
//!     section {
//!         flag = true;
//!         number = 10k;
//!         subsection {
//!             hosts = {
//!                 host = "localhost";
//!                 port = 9000
//!             }
//!             hosts = {
//!                 host = "remotehost"
//!                 port = 9090
//!             }
//!         }
//!     }
//!     ```
//!
//! Differences between UCL and JSON:
//!
//! - outmost braces are optional so `{"a": "b"}` is equivalent to `"a": "b"`
//! - quotes on keys and strings are optional
//! - `:` can be replaced with `=` or even skipped for objects
//! - comma can be replaced with semicolon
//! - trailing commas are allowed
//! - automatic array creation - non-unique keys in object are allowed and are automatically
//!   converted to arrays
//!
//! ## Parser usage
//!
//! Simple example:
//!
//! ```rust
//! static DOC: &'static str = r#"
//! param = value;
//! section {
//!     flag = true;
//!     number = 10k;
//!     subsection {
//!         hosts = {
//!             host = "localhost";
//!             port = 9000
//!         }
//!         hosts = {
//!             host = "remotehost"
//!             port = 9090
//!         }
//!     }
//! }
//! "#;
//!
//! let parser = ucl::Parser::new();
//! let document = parser.parse(DOC).unwrap();
//!
//! assert_eq!(document.fetch("param").unwrap().as_string(), Some("value".to_string()));
//! ```

extern crate libucl_sys;
extern crate libc;
#[macro_use] extern crate bitflags;

pub use error::Error;
pub use parser::Parser;
pub use object::Object;

pub type Result<T> = std::result::Result<T, Error>;

mod utils;
pub mod error;
pub mod parser;
pub mod object;
