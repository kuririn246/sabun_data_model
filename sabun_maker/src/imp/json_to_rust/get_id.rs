use crate::imp::json_to_rust::tmp::tmp_obj::IdValue;
use json5_parser::{JVal};

pub fn get_id(v : &JVal) -> Option<IdValue> {
    match v {
        JVal::String(s, _) => Some(IdValue::Str(s.to_string())),
        JVal::Double(d, _) => Some(IdValue::Num(*d)),
        _ => None
    }
}