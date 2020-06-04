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

    pub fn acceptable(&self, other : &Self) -> bool{
        self.value_type.acceptable(&other.value.qv_type())
    }

    ///null undefined "value" のどれか
    pub fn value_string(&self) -> String{
        match &self.value{
            Qv::Val(s) => format!(r#""{}""#,s.to_string()),
            Qv::Null => "null".to_string(),
            Qv::Undefined => "undefined".to_string(),
        }
    }
}
