use crate::error::Result;
use crate::imp::json_to_rust::validation::validate_list_sabuns::validate_list_sabuns;
use crate::imp::json_to_rust::validation::validate_ref_names::validate_ref_names;
use linked_hash_map::LinkedHashMap;
use crate::imp::json_to_rust::validation::validate_ref::validate_ref;
use crate::imp::json_to_rust::validation::validate_list_def_ref::validate_list_def_ref;
use crate::structs::rust_object::RustObject;
use crate::structs::rust_value::RustValue;

pub fn validate_lists(root : &RustObject) -> Result<()>{
    if root.default.is_none(){ return Ok(()); }

    let root_def = root.default.as_ref().unwrap();
    for (name, value) in root_def{
        let name : &str = name;
        let value : &RustValue = value;

        if let RustValue::List(l) = value {
            let list_def = &l.default;
            //unwrapは絶対に成功するはずだが、データ型はそう言ってないのでデータ型に従ってコーディングする。
            let list_defs_def = list_def.default.as_ref().ok_or_else(|| format!("list {} doesn't have default obj", name))?;

            validate_list_sabuns(name, list_defs_def, &l.list, &l.default.renamed)?;

            if let Some(refs) = &list_def.refs {
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
        if item.refs.is_some(){
            return Some(id);
        }
    }
    return None;
}