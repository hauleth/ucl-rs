use libucl_sys::{
    ucl_object_t,
    ucl_type_t,
    ucl_object_type,
    ucl_object_copy,
    ucl_object_key,
    ucl_object_get_priority,
};

use std::convert::From;
use utils;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Type {
    Object,
    Array,
    Int,
    Float,
    String,
    Boolean,
    Time,
    UserData,
    Null
}

impl From<ucl_type_t> for Type {
    fn from(typ: ucl_type_t) -> Self {
        match typ {
            ucl_type_t::UCL_OBJECT   => Type::Object,
            ucl_type_t::UCL_ARRAY    => Type::Array,
            ucl_type_t::UCL_INT      => Type::Int,
            ucl_type_t::UCL_FLOAT    => Type::Float,
            ucl_type_t::UCL_STRING   => Type::String,
            ucl_type_t::UCL_BOOLEAN  => Type::Boolean,
            ucl_type_t::UCL_TIME     => Type::Time,
            ucl_type_t::UCL_USERDATA => Type::UserData,
            ucl_type_t::UCL_NULL     => Type::Null,
        }
    }
}

pub struct Object {
    obj: *const ucl_object_t,
    typ: Type
}

impl Object {
    pub fn from_cptr(obj: *const ucl_object_t) -> Option<Self> {
        if !obj.is_null() {
            Some(Object {
                obj: obj,
                typ: Type::from(unsafe { ucl_object_type(obj) })
            })
        } else {
            None
        }
    }

    pub fn priority(&self) -> usize {
        unsafe { ucl_object_get_priority(self.obj) as usize }
    }

    pub fn key(&self) -> Option<String> {
        utils::to_str(unsafe { ucl_object_key(self.obj) })
    }

    pub fn get_type(&self) -> Type {
        self.typ
    }

    pub fn as_int(&self) -> Option<i64> {
        use libucl_sys::ucl_object_toint_safe;

        if self.get_type() != Type::Int { return None }

        unsafe {
            let out: *mut i64 = &mut 0i64;
            let res = ucl_object_toint_safe(self.obj, out);

            if res && !out.is_null() {
                Some(*out)
            } else {
                None
            }
        }
    }

    pub fn as_float(&self) -> Option<f64> {
        use libucl_sys::ucl_object_todouble_safe;

        if self.get_type() != Type::Float { return None }

        unsafe {
            let out: *mut f64 = &mut 0f64;
            let res = ucl_object_todouble_safe(self.obj, out);

            if res && !out.is_null() {
                Some(*out)
            } else {
                None
            }
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        use libucl_sys::ucl_object_toboolean_safe;

        if self.get_type() != Type::Boolean { return None }

        unsafe {
            let out: *mut bool = &mut true;
            let res = ucl_object_toboolean_safe(self.obj, out);

            if res && !out.is_null() {
                Some(*out)
            } else {
                None
            }
        }
    }

    pub fn as_string(&self) -> Option<String> {
        use libucl_sys::ucl_object_tostring;

        if self.get_type() != Type::String { return None }

        unsafe {
            let out = ucl_object_tostring(self.obj);

            utils::to_str(out)
        }
    }

    pub fn fetch<T: AsRef<str>>(&self, key: T) -> Option<Object> {
        use libucl_sys::ucl_object_find_key;

        if self.get_type() != Type::Object { return None }

        unsafe {
            let out = ucl_object_find_key(self.obj, utils::to_c_str(key.as_ref()));

            Object::from_cptr(out)
        }
    }

    pub fn fetch_path<T: AsRef<str>>(&self, key: T) -> Option<Object> {
        use libucl_sys::ucl_lookup_path;

        if self.get_type() != Type::Object { return None }

        unsafe {
            let out = ucl_lookup_path(self.obj, utils::to_c_str(key.as_ref()));

            Object::from_cptr(out)
        }
    }
}

impl Clone for Object {
    fn clone(&self) -> Self {
        Object::from_cptr(unsafe { ucl_object_copy(self.obj) }).unwrap()
    }

    fn clone_from(&mut self, other: &Self) {
        self.obj = unsafe { ucl_object_copy(other.obj) };
    }
}

macro_rules! from_primitive {
    ($from: ty => $ctype: ident, $func: ident) => {
        impl From<$from> for Object {
            fn from(val: $from) -> Self {
                use libc;
                use libucl_sys::$func;
                Object::from_cptr(unsafe { $func(val as libc::$ctype) }).unwrap()
            }
        }
    };

    ($from: ty, $func: ident) => {
        impl From<$from> for Object {
            fn from(val: $from) -> Self {
                use libucl_sys::$func;
                Object::from_cptr(unsafe { $func(val) }).unwrap()
            }
        }
    }
}

from_primitive!(i64 => int64_t, ucl_object_fromint);
from_primitive!(f64 => c_double, ucl_object_fromdouble);
from_primitive!(bool, ucl_object_frombool);

impl From<String> for Object {
    fn from(val: String) -> Self {
        use libc;
        use libucl_sys::ucl_object_fromlstring;

        let len = val.len();
        Object::from_cptr(unsafe { ucl_object_fromlstring(utils::to_c_str(val), len as libc::size_t) }).unwrap()
    }
}

impl<'a> From<&'a str> for Object {
    fn from(val: &str) -> Self {
        From::from(val.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_int() {
        let obj = Object::from(10);
        assert_eq!(obj.get_type(), Type::Int);
    }

    #[test]
    fn from_double() {
        let obj = Object::from(10.0f64);
        assert_eq!(obj.get_type(), Type::Float);
    }

    #[test]
    fn from_bool() {
        let obj = Object::from(true);
        assert_eq!(obj.get_type(), Type::Boolean);
    }

    #[test]
    fn from_string() {
        let obj = Object::from("lol".to_string());
        assert_eq!(obj.get_type(), Type::String);
    }

    #[test]
    fn from_str() {
        let obj = Object::from("lol");
        assert_eq!(obj.get_type(), Type::String);
    }

    #[test]
    fn to_int() {
        let obj = Object::from(10);
        assert_eq!(obj.as_int(), Some(10));
    }

    #[test]
    fn to_string() {
        let obj = Object::from("lol");
        assert_eq!(obj.as_string(), Some("lol".to_string()));
    }

    #[test]
    fn to_int_invalid_type() {
        let obj = Object::from(10.0f64);
        assert_eq!(obj.as_int(), None);
    }
}
