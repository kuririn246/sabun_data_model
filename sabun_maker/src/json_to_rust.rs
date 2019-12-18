use serde_json::Value;
use std::collections::BTreeMap;
use crate::rust_struct::{RustValue, RustObject, RustArray, ArrayType};

pub fn json_obj_to_rust(v : &Value) -> Option<RustObject>{
    let v = v.as_object()?;
    let mut r : RustObject = RustObject::new();
    for (k,v) in v{
        match v{
            Value::Bool(b) =>{
                let (is_nullable,name) = is_nullable(k);
                if is_nullable {
                    r.insert(name, RustValue::NullableBool(Some(*b)));
                } else{
                    r.insert(name, RustValue::Bool(*b));
                }
            },
            Value::Array(a) => {
                let (is_nullable, name) = is_nullable(k);

                if let Some(array) = json_array_to_rust(a){
                    if is_nullable{
                        r.insert(name, RustValue::NullableArray(Some(array)));
                    } else{
                        r.insert(name, )
                    }
                }



            },
            _ =>{panic!(); },
        }
        //println!("key {} value {}", k, v);
    }
    Some(r)
}

fn is_nullable(s : &str) -> (bool, String){
    if s.ends_with("?"){
        (true, s[0..s.len()-1].to_string())
    } else{
        (false, s.to_string())
    }
}

fn json_array_to_rust(array : &Vec<Value>, is_nullable : bool) -> Option<RustValue>{
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

    fn get_array(t : ArrayType, a : &Vec<Value>) -> Option<RustArray>{
        let a = &a[1..];
        match t{
            ArrayType::Num =>{ return Some(RustArray{ vec : get_num_array(a)?, array_type : ArrayType::Num }); },
            ArrayType::String{}
        }
    }

    fn get_num_array(a : &[Value]) -> Option<Vec<RustValue::Number>>{
        Some(a.iter().map(|v| RustValue::Number(v.as_f64()?)).collect())
    }

    fn get_str_array(a : &[Value]) -> Option<Vec<RustValue::String>>{
        Some(a.iter().map(|v| RustValue::Number(v.as_f64()?)).collect())
    }

    match get_array_type(a){
        GatResult::AT(a) =>{ return RustValue::Array(get_array(t, a)?); }
    }
    let array = get_array(t, a);
}