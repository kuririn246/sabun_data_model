pub mod names;
pub mod json_obj_to_rust;
pub mod json_name;
pub mod tmp;
pub mod json_item_to_rust;
pub mod json_array_to_rust;
pub mod array_null;
pub mod list;
pub mod get_old;
pub mod get_id;
pub mod get_refs;
// pub mod rename_map;
// pub mod json_dir_to_rust;
pub mod get_include;
// pub mod validation;
//
//
use json5_parser::JVal;
use names::Names;
use crate::error::Result;
use crate::structs::root_object::{RootObject};
use crate::structs::rust_value::RustValue;
use crate::structs::value_type::ValueType;
//
pub fn json_root_to_rust(json : &str) -> Result<RootObject>{
    let jval = json5_parser::from_str(json)?;
    return jval_to_rust_obj(&jval);
}
//
// pub fn json_item_str_to_rust(name : &str, json : &str) -> Result<RustValue>{
//     let jval = json5_parser::from_str(json)?;
//     return json_item_to_rust::json_item_to_rust(name, ValueType::Normal, &jval, &Names::new("."));
// }
//
pub fn jval_to_rust_obj(v : &JVal) -> Result<RootObject> {
    return match v{
        JVal::Map(map, _) =>{
            json_obj_to_rust::json_obj_to_rust(map, false, &Names::new(""))
        },
        _ =>{
            Err(format!(r#"root object is not found"#))?
        }
    };
}

