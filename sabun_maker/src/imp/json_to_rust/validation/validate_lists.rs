use crate::error::Result;
use crate::imp::json_to_rust::validation::validate_list_defaults::validate_list_defaults;
use crate::imp::json_to_rust::validation::validate_ref_names::validate_ref_names;
use linked_hash_map::LinkedHashMap;
use crate::imp::json_to_rust::validation::validate_ref::validate_ref;
use crate::imp::json_to_rust::validation::validate_list_def_ref::validate_list_def_ref;
use crate::structs::rust_object::RustObject;
use crate::structs::rust_value::RustValue;
use crate::imp::json_to_rust::validation::validate_renamed::validate_renamed;
use crate::imp::json_to_rust::names::Names;

pub fn validate_lists(root : &RustObject) -> Result<()>{
    let root_def = &root.default;
    for (name, value) in root_def{
        let name : &str = name;
        let value : &RustValue = value;
        let names = Names::new(name);

        if let RustValue::List(l) = value {
            let list_def = &l.default;
            validate_renamed(list_def, &names.append("Default"))?;

            //unwrapは絶対に成功するはずだが、データ型はそう言ってないのでデータ型に従ってコーディングする。
            let list_defs_def = &list_def.default;
            validate_list_defaults(&names, list_defs_def, &l.list, &l.default.renamed)?;

            if list_def.refs.len() != 0 {
                let refs = &list_def.refs;
                validate_ref_names(name, &l.list, refs, &root.renamed)?;
                validate_ref(name, &l.list, root_def, &root.renamed)?;
                validate_list_def_ref(name, refs, root_def, &root.renamed)?;
            } else{
                if let Some(id) = check_if_items_have_ref(&l.list){
                    Err(format!("{}'s {} has Ref", name, id))?
                }
            }
        }

    }
    return Ok(());
}

fn check_if_items_have_ref(list_items : &LinkedHashMap<String, RustObject>) -> Option<&str>{
    for (id, item) in list_items{
        if item.refs.len() != 0{
            return Some(id);
        }
    }
    return None;
}