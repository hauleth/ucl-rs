use libucl_sys::*;

pub use self::types::Type;
pub use self::builder::Builder;
use utils;

use std::convert::From;
use std::fmt;

pub mod types;
pub mod builder;

#[cfg(test)]
mod test;

/// File element object.
///
/// This structure is immutable typed reference to object inside parsed tree. It can be one of
/// `Type` elements and can be cast only to given type.
pub struct Object {
    obj: *const ucl_object_t,
    typ: Type
}

impl Object {
    /// Create new `Object` form const raw pointer. Internal use only.
    fn from_cptr(obj: *const ucl_object_t) -> Option<Self> {
        if !obj.is_null() {
            Some(Object {
                obj: obj,
                typ: Type::from(unsafe { ucl_object_type(obj) })
            })
        } else {
            None
        }
    }

    // pub fn priority(&self) -> usize {
    //     unsafe { ucl_object_get_priority(self.obj) as usize }
    // }

    /// Return key assigned to object.
    pub fn key(&self) -> Option<String> {
        utils::to_str(unsafe { ucl_object_key(self.obj) })
    }

    /// Return type of object.
    pub fn get_type(&self) -> Type {
        self.typ
    }

    /// Return `i64` value
    ///
    /// # Examples
    ///
    /// ```rust
    /// let obj = ucl::object::Builder::from(10).build();
    /// assert_eq!(obj.as_int(), Some(10));
    ///
    /// let obj = ucl::object::Builder::from("lol").build();
    /// assert_eq!(obj.as_int(), None);
    /// ```
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

    /// Return `f64` value
    ///
    /// # Examples
    ///
    /// ```rust
    /// let obj = ucl::object::Builder::from(10f64).build();
    /// assert_eq!(obj.as_float(), Some(10.0));
    ///
    /// let obj = ucl::object::Builder::from("lol").build();
    /// assert_eq!(obj.as_float(), None);
    /// ```
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

    /// Return boolean value
    ///
    /// # Examples
    ///
    /// ```rust
    /// let obj = ucl::object::Builder::from(true).build();
    /// assert_eq!(obj.as_bool(), Some(true));
    ///
    /// let obj = ucl::object::Builder::from(10).build();
    /// assert_eq!(obj.as_bool(), None);
    /// ```
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

    /// Return string value
    ///
    /// # Examples
    ///
    /// ```rust
    /// let obj = ucl::object::Builder::from("lol").build();
    /// assert_eq!(obj.as_string(), Some("lol".to_string()));
    ///
    /// let obj = ucl::object::Builder::from(10).build();
    /// assert_eq!(obj.as_string(), None);
    /// ```
    pub fn as_string(&self) -> Option<String> {
        use libucl_sys::ucl_object_tostring;

        if self.get_type() != Type::String { return None }

        unsafe {
            let out = ucl_object_tostring(self.obj);

            utils::to_str(out)
        }
    }

    /// Fetch object under key
    ///
    /// # Examples
    ///
    /// ```rust
    /// let obj = ucl::Parser::new().parse("a = b;").unwrap();
    /// assert_eq!(obj.fetch("a").unwrap().as_string(), Some("b".to_string()));
    /// ```
    pub fn fetch<T: AsRef<str>>(&self, key: T) -> Option<Object> {
        use libucl_sys::ucl_object_find_key;

        if self.get_type() != Type::Object { return None }

        unsafe {
            let out = ucl_object_find_key(self.obj, utils::to_c_str(key.as_ref()));

            Object::from_cptr(out)
        }
    }

    /// Fetch object at the end of path delimeted by `.` (dot)
    ///
    /// # Examples
    ///
    /// ```rust
    /// let obj = ucl::Parser::new().parse("a = { b = c; }").unwrap();
    /// assert_eq!(obj.fetch_path("a.b").unwrap().as_string(), Some("c".to_string()));
    /// ```
    pub fn fetch_path<T: AsRef<str>>(&self, path: T) -> Option<Object> {
        use libucl_sys::ucl_lookup_path;

        if self.get_type() != Type::Object { return None }

        unsafe {
            let out = ucl_lookup_path(self.obj, utils::to_c_str(path.as_ref()));

            Object::from_cptr(out)
        }
    }
}

impl fmt::Debug for Object {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let emit = unsafe { ucl_object_emit(self.obj, ucl_emitter::UCL_EMIT_JSON) };
        let string = utils::to_str(emit);

        if string.is_some() {
            fmt.write_str(&string.unwrap())
        } else {
            Err(fmt::Error)
        }
    }
}
