use crate::imp::rust_to_json::rust_array_to_json::rust_array_to_json;
use crate::error::Result;
use crate::imp::rust_to_json::list::rust_list_to_json::rust_list_to_json;
use crate::structs::rust_value::{RustValue, RustParam};
use crate::structs::value_type::ValueType;
use crate::structs::qv::Qv;
use crate::structs::my_json::Value;

pub fn rust_value_to_json_value(v : &RustValue) -> Result<Value>{
    Ok(match v{
        RustValue::Param(RustParam::Bool(b), vt) => to(b, vt, "Bool",|b| Value::Bool(*b)),
        RustValue::Param(RustParam::String(s), vt) => to(s, vt, "Str", |s| Value::String(s.to_string())),
        RustValue::Param(RustParam::Number(n), vt) => to(n, vt, "Num", |n| Value::Number(*n)),
        RustValue::Param(RustParam::Array(a, at), vt) => (rust_array_to_json(a, at, root, name)?, vt.clone()),
        _ =>{ rust_list_to_json(v)? },
    });
}

fn to<T>(qv : &Qv<T>, vt : &ValueType, type_name : &str, f : impl Fn(&T)->Value) -> Value{
    match qv{
        Qv::Val(v) => f(v),
        Qv::Null => Value::Array(vec![Value::String(type_name.to_string()), Value::Null]),
        Qv::Undefined => Value::Array(vec![Value::String(type_name.to_string()), Value::Undefined]),
    }
}