use crate::structs::value_type::ValueType;
use crate::structs::qv::Qv;

#[derive(Debug)]
pub struct RefValue{
    pub value_type : ValueType,
    pub value : Qv<String>,
}

impl RefValue{
    pub fn new(value : Qv<String>, value_type : ValueType) -> RefValue{
        RefValue{ value, value_type }
    }
}
