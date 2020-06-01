use crate::structs::value_type::ValueType;
use crate::structs::qv::Qv;
use crate::indexmap::IndexMap;

#[derive(Debug, PartialEq, Clone)]
pub struct RefValue{
    pub value_type : ValueType,
    pub value : Qv<String>,
}

pub struct RefaMap{
    pub map : IndexMap<String, RefValue>,
    pub is_enum : bool,
}

impl RefValue{
    pub fn new(value : Qv<String>, value_type : ValueType) -> RefValue{
        RefValue{ value, value_type }
    }

    ///nullやundefinedの場合None
    pub fn get_reference(&self) -> Option<&str>{
        match &self.value{
            Qv::Val(s) => Some(s.as_str()),
            Qv::Null => None,
            Qv::Undefined => None,
        }
    }
}
