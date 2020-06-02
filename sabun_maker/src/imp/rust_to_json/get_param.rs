use crate::structs::rust_value::{RustParam};
use crate::structs::qv::Qv;
use crate::structs::my_json::Value;
use crate::imp::rust_to_json::rust_array_to_json::rust_array_to_json;

pub fn get_param(v : &RustParam) -> Value{
    let r = match v{
        RustParam::Bool(b) => to(b,  "Bool",|b| Value::Bool(*b)),
        RustParam::String(s) => to(s, "Str", |s| Value::String(s.str().to_string())),
        RustParam::Number(n)=> to(n, "Num", |n| Value::Number(*n)),
        RustParam::Array(a, at)=>{ rust_array_to_json(a, at) },
    };
    return r;
}

fn to<T>(qv : &Qv<T>, type_name : &str, f : impl Fn(&T)->Value) -> Value{
    match qv{
        Qv::Val(v) => f(v),
        Qv::Null => Value::Array(vec![Value::String(type_name.to_string()), Value::Null]),
        Qv::Undefined => Value::Array(vec![Value::String(type_name.to_string()), Value::Undefined]),
    }
}