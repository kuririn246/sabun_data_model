use std::collections::BTreeMap;
//use crate::indexmap::IndexMap;
//use crate::structs::rust_value::RustValue;
use crate::structs::rust_object::RustObject;
use crate::indexmap::IndexMap;
use crate::structs::rust_value::RustValue;

///auto_idの場合はnewの方は消えるので、oldのitemをアジャストしてnew_listに突っ込む
pub fn adjust_auto_id_item(
    rename : &BTreeMap<String, String>,
    new_list_def : &IndexMap<String, RustValue>,
    old_list_def : &RustObject,
    old : &mut RustObject){
    let mut old = old;

    rename_map(&mut old.default, rename, new_list_def);
    rename_map(&mut old.sabun, rename, new_list_def);


    // if new_list_def.refs.len() != 0 {
    //     adjust_ref( &new_list_def.refs, None, &old_list_def.refs, Some(old.refs))
    // }
}

fn rename_map(map : IndexMap<String, RustValue>, rename : &BTreeMap<String, String>, new_list_def : &IndexMap<String, RustValue>) -> IndexMap<String, RustValue>{
    let mut result = IndexMap::new();

    for (key, value) in map{
        let name = rename.get(&key).unwrap_or(&key);


        if let Some(new_val) = new_list_def.get(name) { //renameで補足されずただメンバがなくなっている場合も

            result.insert(name, value)
        }

    }



    return result;
}
