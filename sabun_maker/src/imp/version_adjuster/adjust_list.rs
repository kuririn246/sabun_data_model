use crate::structs::rust_list::RustList;

use crate::error::Result;
use crate::imp::version_adjuster::rename_old::rename_old;
use crate::imp::json_to_rust::names::Names;
use crate::imp::version_adjuster::adjust_list_item_values::adjust_list_item_values;

pub fn adjust_list(new : &mut RustList, old : RustList, validation : bool, names : &Names) -> Result<()>{
    let mut old = old;

    rename_old(&new.default.renamed, &mut old.default.default);

    let mut new_list = &mut new.list;
    let new_list_def = &new.default.default;
    let renamed = &new.default.renamed;
    let old_list_def = &old.default.default;

    for (name, value) in old.list{
        let redef = &new.redef;
        let name = redef.get(&name).unwrap_or(&name);

        if let Some(item) = new_list.remove(name){
            let mut item = item;
            let sabun = adjust_list_item_values(renamed, new_list_def, &mut item.default, old_list_def, value.default, value.sabun)?;
            item.sabun = sabun;
            new_list.insert(name.to_string(), item);
        }
    }
    return Ok(());
}
