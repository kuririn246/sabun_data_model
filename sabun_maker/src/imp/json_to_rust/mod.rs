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
use std::collections::HashMap;

//
pub fn json_root_to_rust(json : &str) -> Result<RootObject>{
    let jval = json5_parser::from_str(json)?;

    return match jval{
        JVal::Map(map, span) =>{
            let tmp = json_obj_to_rust::json_obj_to_rust(&map, &span, &Names::new(""))?;
            Ok(RootObject{
                include : tmp.include,
                old : tmp.old,
                default : tmp.default,
                sabun : HashMap::new(),
            })
        },
        _ =>{
            Err(format!(r#"root object is not found"#))?
        }
    };
}

