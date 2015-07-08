#![allow(non_camel_case_types)]
#![allow(raw_pointer_derive)]

extern crate libc;
#[macro_use] extern crate bitflags;
extern crate curl_sys;

use libc::{
    c_char,
    c_double,
    c_int,
    c_uint,
    c_uchar,
    c_void,
    int64_t,
    size_t,
    uint16_t,
    uint32_t,
};

#[repr(C)]
#[derive(Clone, Copy)]
pub enum ucl_error_t {
    UCL_EOK = 0,
    UCL_ESYNTAX,
    UCL_EIO,
    UCL_ESTATE,
    UCL_ENESTED,
    UCL_EMACRO,
    UCL_EINTERNAL,
    UCL_ESSL
}

#[repr(C)]
#[derive(Clone, Copy)]
pub enum ucl_type_t {
    UCL_OBJECT = 0,
    UCL_ARRAY,
    UCL_INT,
    UCL_FLOAT,
    UCL_STRING,
    UCL_BOOLEAN,
    UCL_TIME,
    UCL_USERDATA,
    UCL_NULL
}

#[repr(C)]
#[derive(Clone, Copy)]
pub enum ucl_emitter {
    UCL_EMIT_JSON = 0,
    UCL_EMIT_JSON_COMPACT,
    UCL_EMIT_CONFIG,
    UCL_EMIT_YAML
}

bitflags! {
#[repr(C)]
    flags ucl_parser_flags_t: c_int {
        const UCL_PARSER_DEFAULT = 0x0,
        const UCL_PARSER_KEY_LOWERCASE = 0x1,
        const UCL_PARSER_ZEROCOPY = 0x2,
        const UCL_PARSER_NO_TIME = 0x4,
        const UCL_PARSER_NO_IMPLICIT_ARRAYS = 0x8
    }
}

bitflags! {
#[repr(C)]
    flags ucl_string_flags_t : c_int {
        const UCL_STRING_RAW = 0x0,
        const UCL_STRING_ESCAPE = 0x1,
        const UCL_STRING_TRIM = 0x2,
        const UCL_STRING_PARSE_BOOLEAN = 0x4,
        const UCL_STRING_PARSE_INT = 0x8,
        const UCL_STRING_PARSE_DOUBLE = 0x10,
        const UCL_STRING_PARSE_TIME = 0x20,
        const UCL_STRING_PARSE_NUMBER = UCL_STRING_PARSE_INT.bits
            | UCL_STRING_PARSE_DOUBLE.bits
            | UCL_STRING_PARSE_TIME.bits,
        const UCL_STRING_PARSE = UCL_STRING_PARSE_BOOLEAN.bits
            | UCL_STRING_PARSE_NUMBER.bits,
        const UCL_STRING_PARSE_BYTES = 0x40
    }
}

bitflags! {
#[repr(C)]
    flags ucl_object_flags_t: c_int {
        const UCL_OBJECT_ALLOCATED_KEY = 0x1,
        const UCL_OBJECT_ALLOCATED_VALUE = 0x2,
        const UCL_OBJECT_NEED_KEY_ESCAPE = 0x4,
        const UCL_OBJECT_EPHEMERAL = 0x8,
        const UCL_OBJECT_MULTILINE = 0x10,
        const UCL_OBJECT_MULTIVALUE = 0x20
    }
}

#[repr(C)]
pub struct ucl_object_t {
    value: int64_t,
    pub key: *const c_char,
    pub next: *mut ucl_object_t,
    pub prev: *mut ucl_object_t,
    pub keylen: uint32_t,
    pub len: uint32_t,
    pub rc: uint32_t,
    pub flags: uint16_t,
    pub real_type: uint16_t,
    pub trash_stack: [*const c_char; 2]
}

impl ucl_object_t {
    pub unsafe fn iv(&self) -> int64_t { self.value }
    pub unsafe fn sv(&self) -> *const c_char { std::mem::transmute(self.value) }
    pub unsafe fn dv(&self) -> c_double { std::mem::transmute(self.value) }
    pub unsafe fn av(&self) -> *mut c_void { std::mem::transmute(self.value) }
    pub unsafe fn ov(&self) -> *mut c_void { std::mem::transmute(self.value) }
    pub unsafe fn uv(&self) -> *mut c_void { std::mem::transmute(self.value) }
}

pub type ucl_userdata_dtor = extern fn(*mut c_void);
pub type ucl_userdata_emitter = extern fn(*mut c_void) -> *const c_char;
pub type ucl_object_iter_t = *mut c_void;
pub type ucl_macro_handler = extern fn(*const c_uchar, size_t, *const ucl_object_t, *mut c_void) -> bool;
pub type ucl_variable_handler = extern fn(*const c_uchar, size_t, *mut *mut c_uchar, *mut size_t, *mut bool, *mut c_void) -> bool;

#[repr(C)]
pub struct ucl_parser;

#[repr(C)]
pub struct ucl_emitter_functions {
    ucl_emitter_append_character: extern fn(c_uchar, size_t, *mut c_void) -> c_int,
    ucl_emitter_append_len: extern fn(*const c_uchar, size_t, *mut c_void) -> c_int,
    ucl_emitter_append_int: extern fn(int64_t, *mut c_void) -> c_int,
    ucl_emitter_append_double: extern fn(c_double, *mut c_void) -> c_int,
    ucl_emitter_free_func: extern fn(*mut c_void),
    ud: *mut c_void
}

#[repr(C)]
pub struct ucl_emitter_operations {
    ucl_emitter_write_elt: extern fn(*mut ucl_emitter_context, *const ucl_object_t, bool, bool),
    ucl_emitter_start_object: extern fn(*mut ucl_emitter_context, *const ucl_object_t, bool),
    ucl_emitter_end_object: extern fn(*mut ucl_emitter_context, *const ucl_object_t),
    ucl_emitter_start_array: extern fn(*mut ucl_emitter_context, *const ucl_object_t, bool),
    ucl_emitter_end_array: extern fn(*mut ucl_emitter_context, *const ucl_object_t),
}

#[repr(C)]
pub struct ucl_emitter_context {
    name: *const c_char,
    id: c_int,
    func: *const ucl_emitter_functions,
    ops: *const ucl_emitter_operations,
    indent: c_uint,
    top: *const ucl_object_t,
    data: [c_uchar; 1]
}

#[repr(C)]
#[derive(Clone, Copy)]
pub enum ucl_schema_error_code {
    UCL_SCHEMA_OK = 0,
    UCL_SCHEMA_TYPE_MISMATCH,
    UCL_SCHEMA_INVALID_SCHEMA,
    UCL_SCHEMA_MISSING_PROPERTY,
    UCL_SCHEMA_CONSTRAINT,
    UCL_SCHEMA_MISSING_DEPENENCY,
    UCL_SCHEMA_UNKNOWN
}

#[repr(C)]
pub struct ucl_schema_error {
    code: ucl_schema_error_code,
    msg: [c_char; 128],
    obj: *const ucl_object_t
}

extern {
    // Parser functions
    pub fn ucl_parser_new(flags: c_int) -> *mut ucl_parser;
    pub fn ucl_parser_register_macro(parser: *mut ucl_parser, macro_name: *const c_char, handler: ucl_macro_handler, ud: *mut c_void);
    pub fn ucl_parser_register_variable(parser: *mut ucl_parser, var: *const c_char, value: *const c_char);
    pub fn ucl_parser_add_chunk(parser: *mut ucl_parser, data: *const c_char, len: size_t) -> bool;
    pub fn ucl_parser_add_string(parser: *mut ucl_parser, data: *const c_char, len: size_t) -> bool;
    pub fn ucl_parser_add_file(parser: *mut ucl_parser, filename: *const c_char) -> bool;
    pub fn ucl_parser_get_object(parser: *mut ucl_parser) -> *mut ucl_object_t;
    pub fn ucl_parser_get_error(parser: *mut ucl_parser) -> *const c_char;
    pub fn ucl_parser_free(parser: *mut ucl_parser);
    pub fn ucl_parser_set_filevars(parser: *mut ucl_parser, filename: *const c_char, need_expand: bool) -> bool;
    pub fn ucl_parser_set_default_priority(parser: *mut ucl_parser, prio: c_uint) -> bool;
    pub fn ucl_parser_set_variables_handler(parser: *mut ucl_parser, handler: ucl_variable_handler, ud: *mut c_void);
    pub fn ucl_parser_add_chunk_priority(parser: *mut ucl_parser, data: *const c_uchar, len: size_t, prio: c_uint) -> bool;
    pub fn ucl_parser_add_string_priority(parser: *mut ucl_parser, data: *const c_uchar, len: size_t, prio: c_uint) -> bool;
    pub fn ucl_parser_add_file_priority(parser: *mut ucl_parser, filename: *const c_uchar, prio: c_uint) -> bool;
    pub fn ucl_parser_add_fd(parser: *mut ucl_parser, fd: c_int) -> bool;
    pub fn ucl_parser_add_fd_priority(parser: *mut ucl_parser, fd: c_int, prio: c_uint) -> bool;
    pub fn ucl_parser_clear_error(parser: *mut ucl_parser);
    pub fn ucl_parser_get_error_code(parser: *mut ucl_parser) -> c_int;
    pub fn ucl_parser_get_error_column(parser: *mut ucl_parser) -> c_uint;
    pub fn ucl_parser_get_error_linenum(parser: *mut ucl_parser) -> c_uint;

    // Pubkey
    pub fn ucl_pubkey_add(parser: *mut ucl_parser, key: *const c_char, len: size_t) -> bool;

    // Emit functions
    pub fn ucl_object_emit(obj: *const ucl_object_t, emit_type: ucl_emitter) -> *mut c_char;
    // pub fn ucl_object_emit_full(obj: *const ucl_object_t, emit_type: ucl_emitter, ) -> bool;
    // UCL_EXTERN struct ucl_emitter_functions* ucl_object_emit_memory_funcs (
    // UCL_EXTERN struct ucl_emitter_functions* ucl_object_emit_file_funcs (
    // UCL_EXTERN struct ucl_emitter_functions* ucl_object_emit_fd_funcs (
    // UCL_EXTERN void ucl_object_emit_streamline_start_container (
    // UCL_EXTERN void ucl_object_emit_streamline_add_object (
    // UCL_EXTERN void ucl_object_emit_streamline_end_container (
    // UCL_EXTERN void ucl_object_emit_streamline_finish (
    pub fn ucl_object_emit_funcs_free(f: *mut ucl_emitter_functions);
    // UCL_EXTERN struct ucl_emitter_context* ucl_object_emit_streamline_new (

    // Conversion functions
    pub fn ucl_object_toboolean(obj: *const ucl_object_t) -> bool;
    pub fn ucl_object_toboolean_safe(obj: *const ucl_object_t, target: *mut bool) -> bool;
    pub fn ucl_object_todouble(obj: *const ucl_object_t) -> c_double;
    pub fn ucl_object_todouble_safe (obj: *const ucl_object_t, target: *mut c_double) -> bool;
    pub fn ucl_object_toint(obj: *const ucl_object_t) -> int64_t;
    pub fn ucl_object_toint_safe(obj: *const ucl_object_t, target: *mut int64_t) -> bool;
    pub fn ucl_object_tolstring(obj: *const ucl_object_t) -> *const c_char;
    pub fn ucl_object_tostring(obj: *const ucl_object_t) -> *const c_char;
    pub fn ucl_object_tostring_forced(obj: *const ucl_object_t) -> *const c_char;
    pub fn ucl_object_tostring_safe(obj: *const ucl_object_t, target: *mut *const c_char) -> bool;
    pub fn ucl_object_tolstring_safe(obj: *const ucl_object_t, target: *mut *const c_char, len: *mut size_t) -> bool;

    // Generation functions
    pub fn ucl_object_new() -> *mut ucl_object_t;
    pub fn ucl_object_new_full(val: ucl_type_t, prio: c_uint) -> *mut ucl_object_t;
    pub fn ucl_object_typed_new(val: ucl_type_t) -> *mut ucl_object_t;
    pub fn ucl_object_new_userdata(dtor: ucl_userdata_dtor, emitter: ucl_userdata_emitter) -> *mut ucl_object_t;
    pub fn ucl_object_fromint(val: int64_t) -> *mut ucl_object_t;
    pub fn ucl_object_fromdouble(val: c_double) -> *mut ucl_object_t;
    pub fn ucl_object_frombool(val: bool) -> *mut ucl_object_t;
    pub fn ucl_object_fromstring(val: *const c_char) -> *mut ucl_object_t;
    pub fn ucl_object_fromlstring(val: *const c_char, len: size_t) -> *mut ucl_object_t;
    pub fn ucl_object_fromstring_common(val: *const c_char, len: size_t, flags: ucl_string_flags_t) -> *mut ucl_object_t;

    // Utility functions
    pub fn ucl_copy_key_trash(obj: *const ucl_object_t) -> *mut c_char;
    pub fn ucl_copy_value_trash(obj: *const ucl_object_t) -> *mut c_char;
    pub fn ucl_object_copy(other: *const ucl_object_t) -> *mut ucl_object_t;
    pub fn ucl_object_type(obj: *const ucl_object_t) -> ucl_type_t;

    // Object manipulation
    // UCL_EXTERN bool ucl_object_insert_key (ucl_object_t *top, ucl_object_t *elt,
    // UCL_EXTERN bool ucl_object_replace_key (ucl_object_t *top, ucl_object_t *elt,
    pub fn ucl_object_merge(top: *mut ucl_object_t, elt: *mut ucl_object_t, copy: bool) -> bool;
    // UCL_EXTERN bool ucl_object_delete_keyl (ucl_object_t *top,
    // UCL_EXTERN bool ucl_object_delete_key (ucl_object_t *top,
    // UCL_EXTERN ucl_object_t* ucl_object_pop_keyl (ucl_object_t *top, const char *key,
    pub fn ucl_object_pop_key(top: *mut ucl_object_t, key: *const c_char) -> *mut ucl_object_t;
    // UCL_EXTERN bool ucl_object_insert_key_merged (ucl_object_t *top, ucl_object_t *elt,

    // Array manipulation
    // UCL_EXTERN bool ucl_array_append (ucl_object_t *top,
    // UCL_EXTERN bool ucl_array_prepend (ucl_object_t *top,
    // UCL_EXTERN bool ucl_array_merge (ucl_object_t *top, ucl_object_t *elt,
    // UCL_EXTERN ucl_object_t* ucl_array_delete (ucl_object_t *top,
    pub fn ucl_array_head(top: *const ucl_object_t) -> *mut ucl_object_t;
    pub fn ucl_array_tail(top: *const ucl_object_t) -> *mut ucl_object_t;
    pub fn ucl_array_pop_last(top: *mut ucl_object_t) -> *mut ucl_object_t;
    pub fn ucl_array_pop_first(top: *mut ucl_object_t) -> *mut ucl_object_t;
    pub fn ucl_array_find_index(top: *const ucl_object_t, index: c_uint) -> *const ucl_object_t;
    // UCL_EXTERN unsigned int ucl_array_index_of (ucl_object_t *top,

    // Iteration functions
    pub fn ucl_iterate_object(obj: *const ucl_object_t, iter: *mut ucl_object_iter_t, expand_values: bool) -> *const ucl_object_t;
    pub fn ucl_object_iterate_new(obj: *const ucl_object_t) -> ucl_object_iter_t;
    pub fn ucl_object_iterate_reset(it: ucl_object_iter_t, obj: *const ucl_object_t) -> ucl_object_iter_t;
    pub fn ucl_object_iterate_safe(iter: ucl_object_iter_t, expand_values: bool) -> *const ucl_object_t;
    pub fn ucl_object_iterate_free(it: ucl_object_iter_t);

    // UCL_EXTERN ucl_object_t * ucl_elt_append (ucl_object_t *head,
    pub fn ucl_object_find_key(obj: *const ucl_object_t, key: *const c_char) -> *const ucl_object_t;
    // UCL_EXTERN const ucl_object_t* ucl_object_find_keyl (const ucl_object_t *obj,
    pub fn ucl_lookup_path(obj: *const ucl_object_t, path: *const c_char) -> *const ucl_object_t;
    // UCL_EXTERN const ucl_object_t *ucl_lookup_path_char (const ucl_object_t *obj,
    pub fn ucl_object_key (obj: *const ucl_object_t) -> *const c_char;
    pub fn ucl_object_keyl(obj: *const ucl_object_t, len: *mut size_t) -> *const c_char;
    pub fn ucl_object_ref(obj: *const ucl_object_t) -> *mut ucl_object_t;
    // UCL_DEPRECATED(UCL_EXTERN void ucl_object_free (ucl_object_t *obj));
    pub fn ucl_object_unref(obj: *mut ucl_object_t);
    // UCL_EXTERN int ucl_object_compare (const ucl_object_t *o1,
    // UCL_EXTERN void ucl_object_array_sort (ucl_object_t *ar,
    pub fn ucl_object_get_priority(obj: *const ucl_object_t) -> c_uint;
    // UCL_EXTERN void ucl_object_set_priority (ucl_object_t *obj,
    // UCL_EXTERN bool ucl_object_validate (const ucl_object_t *schema,
}
