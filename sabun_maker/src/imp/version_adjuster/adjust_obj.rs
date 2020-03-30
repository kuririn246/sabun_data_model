use crate::structs::rust_object::RustObject;
use crate::imp::version_adjuster::construct_new_sabun::construct_new_sabun;
use crate::error::Result;
use crate::structs::rust_value::RustValue;
use std::collections::HashMap;
use crate::imp::json_to_rust::validation::validate_root::validate_root;

pub fn adjust_obj(new : RustObject, old : RustObject, validation : bool) -> Result<RustObject>{
    let mut new = new;
    let mut old = old;
    let new_sabun = construct_new_sabun(&new.renamed, old.sabun);

    //sabunはold versionを受け入れる
    new.sabun = new_sabun;

    if old.default.is_none() || new.default.is_none(){
        return Ok(new);
    }

    let new_default = &new.default.unwrap();
    let new_default : &HashMap<String, RustValue> = new_default;
    let result_default : HashMap<String, RustValue> = HashMap::new();

    for (key, value) in &old.default.unwrap(){
        if let RustValue::List(l) = value{
            let rename_map = &new.renamed;
            let name = rename_map.get(key).unwrap_or(key);

            if let Some(val) = new_default.get(name){
                if let RustValue::List(new_l) = val{

                } else{
                    //新バージョンではListじゃなくなっていた・・・？　しょうがないからなにもせず新バージョンを受け入れるしかないだろう
                }
            }
        }
    }

    if validation {
        validate_root(&new)?;
    }

    return Ok(new);
}