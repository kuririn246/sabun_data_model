use crate::imp::rust_to_json::get_param::get_param;
use crate::imp::structs::array_type::ArrayType;
use crate::imp::structs::qv::Qv;
use crate::imp::structs::my_json::Value;
use crate::imp::structs::rust_array::RustArray;

pub fn rust_array_to_json(array : &RustArray, at : &ArrayType) -> Value{
    let mut result : Vec<Value> = vec![];


    match at{
        ArrayType::String =>{ result.push(Value::String("StrArray".to_string())) },
        ArrayType::Num =>{
            let array_len = if let Qv::Val(v) = array.qv(){
                v.len()
            } else{ 0 };
            //noisyすぎるので基本省略
            if array_len == 0 {
                result.push(Value::String("NumArray".to_string()))
            }
        }
        ArrayType::Num2 =>{ result.push(Value::String("Num2Array".to_string())) }
    }
    match array.qv(){
        Qv::Val(v) => {
            for item in v{
                result.push(get_param(item));
            }
        },
        Qv::Undefined =>{ result.push(Value::Undefined) },
        Qv::Null =>{ result.push(Value::Null) },
    }
    return Value::Array(result);
}