use serde_json::Value;
use std::collections::BTreeMap;
use crate::rust_struct::{RustValue, RustObject, RustArray};

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
            Value::Array(a) => {},
            _ =>{},
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

fn json_array_to_rust(array : &Vec<Value>) -> Option<RustArray>{
    None
}