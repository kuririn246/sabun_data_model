use serde_json::Value;
use crate::rust_struct::{RustValue, ArrayType, RustArray, Qv};
use crate::json_to_rust::Names;
use crate::json_list_to_rust::json_list_to_rust;

pub fn json_array_to_rust(array : &Vec<Value>, is_nullable : bool, names : &Names) -> Result<RustValue, String>{
    match get_array_type(array){
        GatResult::AT(array_type) =>{
            return Ok(get_array(array_type, array, is_nullable, names)?);
        },
        GatResult::None =>{ return Err(format!(r#"Array must be "...-Array" or "List" "#)); },
        GatResult::List =>{
            Ok(json_list_to_rust(array, is_nullable, names)?);
        },
    }
}

enum GatResult{
    AT(ArrayType),
    List,
    None
}

fn get_array_type(a : &Vec<Value>) -> GatResult{
    if let Some(v) = a.get(0){
        if let Some(s) = v.as_str(){
            match s{
                "Num-Array" =>{ return GatResult::AT(ArrayType::Num); },
                "Str-Array" =>{ return GatResult::AT(ArrayType::String); },
                "Num-Array2" =>{ return GatResult::AT(ArrayType::Num2); }
                "List" => { return GatResult::List; }
                _=>{ return GatResult::None; }
            }
        }
    }
    GatResult::None
}

fn get_array(t : ArrayType, a : &Vec<Value>, is_nullable : bool, names : &Names) -> Result<RustValue, String>{
    let a = &a[1..];
    match t{
        ArrayType::Num =>{
            let array = get_num_array(a, names)?;
            if is_nullable{
                return Ok(RustValue::NullableArray(Qv::Val(Some(array))));
            } else{
                return Ok(RustValue::Array(Qv::Val(array)));
            }
        },
        ArrayType::String =>{
            let array = get_str_array(a, names)?;
            if is_nullable{
                return Ok(RustValue::NullableArray(Qv::Val(Some(array))));
            } else{
                return Ok(RustValue::Array(array));
            }
        },
        ArrayType::Num2 =>{
            let array = get_num_array2(a, names)?;
            if is_nullable{
                return Ok(RustValue::NullableArray(Some(array)));
            } else{
                return Ok(RustValue::Array(array));
            }
        },
    }
}

fn get_num_array(a : &[Value], names : &Names) -> Result<RustArray, String>{
    let mut vec : Vec<RustValue> = vec![];
    for item in a{
        vec.push(RustValue::Number(item.as_f64().ok_or(format!("{} is not a number {}", item, names.to_string()))?));
    }
    return Ok(RustArray{ vec, array_type : ArrayType::Num });
}

fn get_str_array(a : &[Value], names : &Names) -> Result<RustArray, String>{
    let mut vec : Vec<RustValue> = vec![];
    for item in a{
        let s = item.as_str().ok_or(format!("{} is not string {}", item, names.to_string()))?;
        vec.push(RustValue::String(s.to_string()));
    }
    return Ok(RustArray{ vec, array_type : ArrayType::String });
}

fn get_num_array2(a : &[Value], names : &Names) -> Result<RustArray, String>{
    let mut vec : Vec<RustValue> = vec![];
    for item in a{
        match item{
            Value::Array(a) =>{
                let array = get_num_array(a, names)?;
                vec.push(RustValue::Array(array));
            }
            _=>{ return Err(format!("{} is not an array {}", item, names.to_string())); }
        }
    }
    return Ok(RustArray{ vec, array_type : ArrayType::Num2 })
}