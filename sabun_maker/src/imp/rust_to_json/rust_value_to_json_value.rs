use crate::my_json::Value;
use crate::rust_struct::{RustValue, ValueType, Qv};

pub fn rust_value_to_json_value(v : &RustValue) -> (Value, ValueType){
    return match v{
        RustValue::Bool(b, vt) => to(b, vt, "Bool",|b| Value::Bool(*b)),
        RustValue::String(s, vt) => to(s, vt, "Str", |s| Value::String(s.to_string())),
        RustValue::Number(n, vt) => to(n, vt, "Num", |n| Value::Num(*n)),
    }

}

fn to<T>(qv : &Qv<T>, vt : &ValueType, type_name : &str, f : impl Fn(&T)->Value) -> (Value, ValueType){
    match qv{
        Qv::Val(v) => (f(v), vt.clone()),
        Qv::Null => (Value::Array(vec![Value::String(type_name.to_string()), Value::Null]), vt.clone()),
        Qv::Undefined => (Value::Array(vec![Value::String(type_name.to_string()), Value::Undefined]), vt.clone()),
    }
}