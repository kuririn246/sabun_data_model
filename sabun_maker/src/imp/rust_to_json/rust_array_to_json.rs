use crate::error::Result;
use crate::structs::qv::Qv;
use crate::structs::array_type::ArrayType;
use crate::structs::rust_value::RustArray;
use crate::structs::my_json::Value;

pub fn rust_array_to_json(qv : &Qv<RustArray>, at : &ArrayType) -> Result<Value>{
    // let mut result : Vec<Value> = vec![];
    // match at{
    //     ArrayType::String =>{ result.push(Value::String("StrArray".to_string())) },
    //     ArrayType::Num =>{ result.push(Value::String("NumArray".to_string())) }
    //     ArrayType::Num2 =>{ result.push(Value::String("Num2Array".to_string())) }
    // }
    // match qv{
    //     Qv::Val(v) => {
    //         for item in &v.vec{
    //             let (v, _) = rust_value_to_json_value(item, root, name)?;
    //             result.push(v);
    //         }
    //     },
    //     Qv::Undefined =>{ result.push(Value::Undefined) },
    //     Qv::Null =>{ result.push(Value::Null) },
    // }
    //return Ok(Value::Array(result));
    todo!()
}