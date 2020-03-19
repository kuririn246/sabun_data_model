use crate::rust_struct::{Qv, RustArray, RustValue, ArrayType, ValueType};
use crate::my_json::Value;
use crate::imp::rust_to_json::rust_value_to_json_value::rust_value_to_json_value;
use crate::error::Result;

pub fn rust_array_to_json(qv : &Qv<RustArray>, at : &ArrayType, name : &str) -> Result<Value>{
    let mut result : Vec<Value> = vec![];
    match at{
        ArrayType::String =>{ result.push(Value::String("Str-Array".to_string())) },
        ArrayType::Num =>{ result.push(Value::String("Num-Array".to_string())) }
        ArrayType::Num2 =>{ result.push(Value::String("Num2-Array".to_string())) }
    }
    match qv{
        Qv::Val(v) => {
            for item in &v.vec{
                let (v, _) = rust_value_to_json_value(item, name);
                result.push(v);
            }
        },
        Qv::Undefined =>{ result.push(Value::Undefined) },
        Qv::Null =>{ result.push(Value::Null) },
    }
    return Ok(Value::Array(result));
}