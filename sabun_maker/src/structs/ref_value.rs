use crate::structs::value_type::ValueType;
use crate::structs::qv::Qv;

#[derive(Debug, PartialEq)]
pub struct RefValue{
    pub value_type : ValueType,
    pub default_value : Qv<String>,
    pub sabun_value : Option<Qv<String>>,
}

impl RefValue{
    pub fn new(default_value : Qv<String>, value_type : ValueType) -> RefValue{
        RefValue{ default_value, value_type, sabun_value : None }
    }

    pub fn get_value(&self) -> &Qv<String>{
        if let Some(val) = self.sabun_value.as_ref(){
            val
        } else {
            &self.default_value
        }
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
