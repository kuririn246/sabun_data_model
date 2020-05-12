use crate::structs::rust_list::RustList;
use crate::imp::json_to_rust::names::Names;
use std::collections::BTreeMap;
use crate::error::Result;
use crate::imp::version_adjuster::rename_ref::rename_ref;
use crate::imp::version_adjuster::adjust_auto_id_items_ref::adjust_auto_id_items_ref;
use crate::structs::rust_object::RustObject;

pub fn adjust_auto_id_list(new : &mut RustList, old : RustList, root_rename : &BTreeMap<String, String>, names : &Names) -> Result<()>{
    let mut old = old;

    //rename_old(&new.default.renamed, &mut old.default.default);
    rename_ref(root_rename, &mut old.default.refs);

    new.list.clear();

    for (key, value) in old.list{
        let mut obj = RustObject::new();
        obj.refs = adjust_auto_id_items_ref(root_rename, &new.default.refs, &old.default.refs, &value.refs);
        obj.obsolete = value.obsolete;
        obj.id = value.id.clone();

        new.list.insert(key.to_string(), obj);
    }

    // let mut new_list = &mut new.list;
    // let new_list_def = &new.default.default;
    // let mut renamed = &new.default.renamed;
    // let old_list_def = &old.default.default;
    //
    // for (name, value) in old.list{
    //     let redef = &new.redef;
    //     let name = redef.get(&name).unwrap_or(&name);
    //
    //     if let Some(item) = new_list.remove(name){
    //         let mut item = item;
    //         let sabun = adjust_list_item_values(renamed, new_list_def, &mut item.default, old_list_def, value.default, value.sabun)?;
    //         item.sabun = sabun;
    //         new_list.insert(name.to_string(), item);
    //     }
    // }
    return Ok(());
}