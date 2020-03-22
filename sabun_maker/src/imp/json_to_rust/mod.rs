pub mod names;
pub mod json_obj_to_rust;
pub mod json_name;
pub mod json_item_to_rust;
pub mod json_array_to_rust;
pub mod array_null;
pub mod list;
pub mod get_renamed;
pub mod get_refs;
pub mod rename_map;
pub mod json_dir_to_rust;

use crate::rust_struct::{RustObject};

use json5_parser::JVal;
use names::Names;
use crate::error::Result;

pub fn json_to_rust(v : &JVal) -> Result<RustObject> {
    return match v{
        JVal::Map(map, _) =>{
            json_obj_to_rust::json_obj_to_rust(map, false, &Names::new(""))
        },
        _ =>{
            Err(format!(r#"root object is not found"#))?
        }
    };
}

//
//
