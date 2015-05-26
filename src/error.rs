use libucl_sys::ucl_error_t;

#[derive(Clone, Copy, Debug)]
pub enum Error {
    Ok,
    Syntax,
    Io,
    State,
    Nested,
    Macro,
    Internal,
    SSL
}

impl From<ucl_error_t> for Error {
    fn from(err: ucl_error_t) -> Self {
        match err {
            ucl_error_t::UCL_EOK       => Error::Ok,
            ucl_error_t::UCL_ESYNTAX   => Error::Syntax,
            ucl_error_t::UCL_EIO       => Error::Io,
            ucl_error_t::UCL_ESTATE    => Error::State,
            ucl_error_t::UCL_ENESTED   => Error::Nested,
            ucl_error_t::UCL_EMACRO    => Error::Macro,
            ucl_error_t::UCL_EINTERNAL => Error::Internal,
            ucl_error_t::UCL_ESSL      => Error::SSL
        }
    }
}
