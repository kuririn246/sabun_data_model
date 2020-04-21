use std::collections::BTreeMap;
use crate::structs::rust_value::RustValue;
use crate::indexmap::IndexMap;
use crate::error::Result;
use crate::imp::json_to_rust::names::Names;

pub fn rename_old(rename : &BTreeMap<String, String>, old : &mut IndexMap<String, RustValue>) {
    for (old_name, new_name) in rename{
        if let Some(val) = old.remove(old_name){
            if old.contains_key(new_name) == false {
                //新しいのがすでにある場合はあるものを優先でいいんじゃないですかね・・・？
                old.insert(new_name.to_string(), val);
            }
        }
    }
}