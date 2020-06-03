use crate::structs::root_object::{ListDefObj, RootObject};
use std::collections::HashMap;
use crate::structs::rust_list::ListItem;
use crate::error::Result;
use crate::imp::json_to_rust::names::Names;

pub fn validate_data(def : &ListDefObj, data_map : &HashMap<String, ListItem>, root : &RootObject, names : &Names) -> Result<()>{
    def.
}