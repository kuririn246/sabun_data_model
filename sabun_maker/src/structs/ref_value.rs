use crate::structs::value_type::ValueType;
use crate::structs::qv::Qv;

#[derive(Debug, PartialEq, Clone)]
pub struct RefValue{
    value_type : ValueType,
    value : Qv<String>,
}

impl RefValue{
    pub fn new(value : Qv<String>, value_type : ValueType) -> RefValue{
        RefValue{ value, value_type }
    }
    pub fn value_type(&self) -> ValueType{ self.value_type }
    pub fn value(&self) -> &Qv<String>{ &self.value }

    ///nullやundefinedの場合None
    pub fn get_reference(&self) -> Option<&str>{
        match &self.value{
            Qv::Val(s) => Some(s.as_str()),
            Qv::Null => None,
            Qv::Undefined => None,
        }
    }
}
