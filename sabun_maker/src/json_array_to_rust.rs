use serde_json::Value;
use crate::rust_struct::{RustValue, ArrayType, RustArray};

pub fn json_array_to_rust(array : &Vec<Value>, is_nullable : bool) -> Option<RustValue>{
    match get_array_type(array){
        GatResult::AT(array_type) =>{ return Some(get_array(array_type, array, is_nullable)?); }
        GatResult::None =>{ return None; },
        GatResult::List =>{
            //TODO: implement List
            return None;
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

fn get_array(t : ArrayType, a : &Vec<Value>, is_nullable : bool) -> Option<RustValue>{
    let a = &a[1..];
    match t{
        ArrayType::Num =>{
            let array = get_num_array(a)?;
            if is_nullable{
                return Some(RustValue::NullableArray(Some(array)));
            } else{
                return Some(RustValue::Array(array));
            }
        },
        ArrayType::String =>{
            let array = get_str_array(a)?;
            if is_nullable{
                return Some(RustValue::NullableArray(Some(array)));
            } else{
                return Some(RustValue::Array(array));
            }
        },
        ArrayType::Num2 =>{
            let array = get_num_array2(a)?;
            if is_nullable{
                return Some(RustValue::NullableArray(Some(array)));
            } else{
                return Some(RustValue::Array(array));
            }
        },
    }
}

fn get_num_array(a : &[Value]) -> Option<RustArray>{
    let mut vec : Vec<RustValue> = vec![];
    for item in a{
        vec.push(RustValue::Number(item.as_f64()?));
    }
    return Some(RustArray{ vec, array_type : ArrayType::Num });
}

fn get_str_array(a : &[Value]) -> Option<RustArray>{
    let mut vec : Vec<RustValue> = vec![];
    for item in a{
        vec.push(RustValue::String(item.as_str()?.to_string()));
    }
    return Some(RustArray{ vec, array_type : ArrayType::String });
}

fn get_num_array2(a : &[Value]) -> Option<RustArray>{
    let mut vec : Vec<RustValue> = vec![];
    for item in a{
        match item{
            Value::Array(a) =>{
                let array = get_num_array(a)?;
                vec.push(RustValue::Array(array));
            }
            _=>{ return None; }
        }
    }
    return Some(RustArray{ vec, array_type : ArrayType::Num2 })
}