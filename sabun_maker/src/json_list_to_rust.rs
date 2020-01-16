use serde_json::Value;
use crate::json_to_rust::Names;
use crate::rust_struct::RustValue;

pub fn json_list_to_rust(vec : &Vec<Value>, is_nullable : bool, names : &Names) -> Result<RustValue, String>{
    let s = &vec[1..];

    for item in s{
        match item{
            &Value::Array(ref v) =>{  }
        }
    }

    todo!();
}

//fn get_list_array(v : &Vec<Value>) -> Result<(), String>{
//
//}