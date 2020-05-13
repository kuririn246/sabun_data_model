use crate::my_json::Value;
use crate::imp::rust_to_json::rust_array_to_json::rust_array_to_json;
use crate::error::Result;
use crate::imp::rust_to_json::list::rust_list_to_json::rust_list_to_json;
use crate::structs::rust_value::RustValue;
use crate::structs::value_type::ValueType;
use crate::structs::qv::Qv;
use crate::structs::rust_object::RustObject;

pub fn rust_value_to_json_value(v : &RustValue, root : &RustObject, name : &str) -> Result<(Value, ValueType)>{
    let r = match v{
        RustValue::Bool(b, vt) => to(b, vt, "Bool",|b| Value::Bool(*b)),
        RustValue::String(s, vt) => to(s, vt, "Str", |s| Value::String(s.to_string())),
        RustValue::Number(n, vt) => to(n, vt, "Num", |n| Value::Number(*n)),
        RustValue::Array(a, at, vt) => (rust_array_to_json(a, at, name)?, vt.clone()),
        RustValue::List(l)=> {
            (rust_list_to_json(l, root, name)?, ValueType::Normal)
        },
        // RustValue::Object(_o) =>{
        //     //仕様上unreachable。むりやり書こうとしても[obj,null]の記法がないからかけないな・・・
        //     Err(format!("{} objects must not have objects", name))?
        // },
    };
    return Ok(r);
}

fn to<T>(qv : &Qv<T>, vt : &ValueType, type_name : &str, f : impl Fn(&T)->Value) -> (Value, ValueType){
    match qv{
        Qv::Val(v) => (f(v), vt.clone()),
        Qv::Null => (Value::Array(vec![Value::String(type_name.to_string()), Value::Null]), vt.clone()),
        Qv::Undefined => (Value::Array(vec![Value::String(type_name.to_string()), Value::Undefined]), vt.clone()),
    }
}