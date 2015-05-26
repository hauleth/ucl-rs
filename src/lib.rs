extern crate libucl_sys;
extern crate libc;

pub use error::Error;
pub use parser::Parser;
pub use object::Object;

pub type Result<T> = std::result::Result<T, Error>;

mod utils;
pub mod error;
pub mod parser;
pub mod object;

#[test]
fn it_works() {
}
