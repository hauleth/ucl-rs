use libucl_sys::ucl_type_t;

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
