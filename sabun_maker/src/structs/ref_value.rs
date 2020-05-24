use crate::structs::value_type::ValueType;
use crate::structs::qv::Qv;

#[derive(Debug, PartialEq, Clone)]
pub struct RefValue{
    pub value_type : ValueType,
    pub default_value : Qv<String>,
}

impl RefValue{
    pub fn new(default_value : Qv<String>, value_type : ValueType) -> RefValue{
        RefValue{ default_value, value_type }
    }

    ///nullやundefinedの場合None
    pub fn get_reference(&self) -> Option<&str>{
        match self.get_value(){
            Qv::Val(s) => Some(s.as_str()),
            Qv::Null => None,
            Qv::Undefined => None,
        }
    }
}
