use std::collections::BTreeMap;
//use crate::indexmap::IndexMap;
//use crate::structs::rust_value::RustValue;
use crate::structs::rust_object::RustObject;
use crate::imp::version_adjuster::rename_old::rename_old;
use crate::imp::version_adjuster::adjust_ref::adjust_ref;

///auto_idの場合はnewの方は消えるので、oldのitemをアジャストしてnew_listに突っ込む
pub fn adjust_auto_id_item(
    renamed : &BTreeMap<String, String>,
    new_list_def : &RustObject,
    old_list_def : &RustObject,
    old : RustObject){
    let mut old = old;




    // if new_list_def.refs.len() != 0 {
    //     adjust_ref( &new_list_def.refs, None, &old_list_def.refs, Some(old.refs))
    // }
}
