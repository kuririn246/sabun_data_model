use crate::imp::json_to_rust::tmp::tmp_obj::IdValue;
use crate::error::Result;
use json5_parser::{JVal, Span};
use crate::imp::json_to_rust::json_name::json_simple_name;

pub fn get_id(v : &JVal) -> Option<IdValue> {
    match v {
        JVal::String(s, _) => Some(IdValue::Str(s.to_string())),
        JVal::Double(d, _) => Some(IdValue::Num(*d)),
        _ => None
    }
}