pub mod names;
pub mod json_obj_to_rust;
pub mod json_name;
pub mod json_item_to_rust;
pub mod json_array_to_rust;
pub mod array_null;
pub mod list;
use crate::rust_struct::{RustObject};
//use json_name::{json_name, NameType, SystemNames};
//use crate::json_item_to_rust::json_item_to_rust;
//use crate::get_ref_ids::get_ref_ids;
//use crate::get_rename::get_rename;
use std::collections::BTreeMap;

use json5_parser::JVal;
use names::Names;

pub fn json_to_rust(v : &JVal) -> Result<RustObject,String> {
    let v = v.as_object().ok_or("v is not an object")?;
    todo!();
    //return Ok(json_obj_to_rust2(v, &Names::new("")).unwrap());
}

//
//
