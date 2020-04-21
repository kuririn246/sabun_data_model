use std::collections::BTreeMap;
use crate::structs::rust_object::RustObject;
use crate::structs::ref_value::RefValue;
use crate::indexmap::IndexMap;

pub fn rename_ref(
    new_list_def_rename : &BTreeMap<String, String>,
    new_def : &IndexMap<String, RefValue>,
    new_list : &mut LinkedHashMap<String, RustObject>,
    old_list_def_rename : &BTreeMap<String, String>,
    old_def : &IndexMap<String, RefValue>,
    old_list : &mut LinkedHashMap<String, RustObject>,
    root : &RustObject){

    let root_rename = &root.renamed;


}

fn rename_value(rename : &BTreeMap<String, String>){

}