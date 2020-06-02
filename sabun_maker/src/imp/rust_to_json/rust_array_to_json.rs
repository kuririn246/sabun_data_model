use crate::structs::qv::Qv;
use crate::structs::array_type::ArrayType;
use crate::structs::rust_value::RustArray;
use crate::structs::my_json::Value;
use crate::imp::rust_to_json::get_param::get_param;

pub fn rust_array_to_json(qv : &Qv<RustArray>, at : &ArrayType) -> Value{
    let mut result : Vec<Value> = vec![];


    match at{
        ArrayType::String =>{ result.push(Value::String("StrArray".to_string())) },
        ArrayType::Num =>{
            let array_len = if let Qv::Val(v) = qv{
                v.vec().len()
            } else{ 0 };
            //noisyすぎるので基本省略
            if array_len == 0 {
                result.push(Value::String("NumArray".to_string()))
            }
        }
        ArrayType::Num2 =>{ result.push(Value::String("Num2Array".to_string())) }
    }
    match qv{
        Qv::Val(v) => {
            for item in v.vec(){
                result.push(get_param(item));
            }
        },
        Qv::Undefined =>{ result.push(Value::Undefined) },
        Qv::Null =>{ result.push(Value::Null) },
    }
    return Value::Array(result);
}