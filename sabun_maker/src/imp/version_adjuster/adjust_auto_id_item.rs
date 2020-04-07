use std::collections::BTreeMap;
use crate::indexmap::IndexMap;
use crate::structs::rust_value::RustValue;
use crate::structs::rust_object::RustObject;
use crate::imp::version_adjuster::rename_old::rename_old;
use crate::imp::version_adjuster::adjust_ref::adjust_ref;

pub fn adjust_auto_id_item(
    renamed : &BTreeMap<String, String>,
    new_list_def : &RustObject,
    old : &mut RustObject){


    rename_old(renamed, &mut old.default)
    rename_old(renamed, &mut old.sabun);

    if new_list_def.refs.is_some() {
        adjust_ref(renamed, new_list_def.refs.as_ref().unwrap(), None, )
    }
}
