use std::collections::BTreeMap;
use crate::structs::rust_value::RustValue;
use crate::error::Result;

pub fn adjust_list_item_values(
    new_list_def : BTreeMap<String, RustValue>,
    new : BTreeMap<String, RustValue>,
    old_list_def : BTreeMap<String, RustValue>,
    old_def : BTreeMap<String, RustValue>,
    sabun : BTreeMap<String, RustValue>) -> Result<BTreeMap<String, RustValue>>{


    return Ok(new);

}