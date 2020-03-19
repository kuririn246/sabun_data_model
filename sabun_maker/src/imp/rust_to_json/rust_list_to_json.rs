use crate::rust_struct::{Qv, RustValue, ArrayType, ValueType, RustList};
use crate::my_json::Value;
use crate::imp::rust_to_json::rust_value_to_json_value::rust_value_to_json_value;
use crate::error::Result;

pub fn rust_list_to_json(qv : &Qv<RustList>, at : &ArrayType) -> Value{
    let mut result : Vec<Value> = vec![];
    
    match qv{
        Qv::Val(v) => {
            for item in &v.vec{
                let (v, _) = rust_value_to_json_value(item);
                result.push(v);
            }
        },
        Qv::Undefined =>{ result.push(Value::Undefined) },
        Qv::Null =>{ result.push(Value::Null) },
    }
    return Value::Array(result);
}