use libucl_sys::ucl_error_t;

#[derive(Clone, Debug)]
pub enum Error {
    Ok,
    Syntax(String),
    Io,
    State,
    Nested,
    Macro,
    Internal,
    SSL,
    Other
}

impl Error {
    pub fn from_code(num: i32, desc: String) -> Self {
        match num {
            _ if num == ucl_error_t::UCL_EOK       as i32 => Error::Ok,
            _ if num == ucl_error_t::UCL_ESYNTAX   as i32 => Error::Syntax(desc),
            _ if num == ucl_error_t::UCL_EIO       as i32 => Error::Io,
            _ if num == ucl_error_t::UCL_ESTATE    as i32 => Error::State,
            _ if num == ucl_error_t::UCL_ENESTED   as i32 => Error::Nested,
            _ if num == ucl_error_t::UCL_EMACRO    as i32 => Error::Macro,
            _ if num == ucl_error_t::UCL_EINTERNAL as i32 => Error::Internal,
            _ if num == ucl_error_t::UCL_ESSL      as i32 => Error::SSL,
            _ => Error::Other,
        }
    }
}
