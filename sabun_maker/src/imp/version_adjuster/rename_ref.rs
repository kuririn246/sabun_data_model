use std::collections::BTreeMap;
//use crate::structs::rust_object::RustObject;
use crate::structs::ref_value::RefValue;
use crate::indexmap::IndexMap;

pub fn rename_ref(rename : &BTreeMap<String, String>, old : &mut IndexMap<String, RefValue>){
    for (key,value) in rename{
        if let Some(val) = old.remove(key){
            if old.contains_key(value) == false{
                old.insert(value.to_string(), val);
            }
        }
    }
}
