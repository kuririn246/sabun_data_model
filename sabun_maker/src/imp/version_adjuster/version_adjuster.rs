use crate::imp::json_to_rust::validation::validate_root::validate_root;
use crate::imp::version_adjuster::adjust_mut_list::adjust_mut_list;
use std::collections::HashMap;
use crate::error::Result;
use crate::imp::json_to_rust::names::Names;
use crate::imp::structs::root_obj::RootObject;
use crate::imp::structs::root_value::RootValue;

/// paramのsabunがあれば上書き、mut_listはoldのものを全部入れ、（あるなら）newの方のものは全削除して入れ替える
/// 基本的に、新バージョンのjsonと旧バージョンのデータが有り、旧バージョンのデータはRootにsabun、MutListは追加が行われていることが想定されている
/// Json段階ではMutListにアイテムは入れられない建前なので、出来る変化はMutListの追加とRootの差分だけのはず
/// Defaultが更新されるので、undefinedが設定され、
pub fn adjust_versions(new : RootObject, old : RootObject, validation : bool) -> Result<RootObject>{

    let (def, sabun, old_hash) = new.deconstruct();
    let mut sabun = sabun;
    let mut new_map :HashMap<String, RootValue> = HashMap::with_capacity(def.len());

    let (old_def,old_sabun, _) = old.deconstruct();
    let mut old_sabun = old_sabun;
    let mut old_def = old_def;

    for (def_key, def_value) in def{
        match def_value{
            RootValue::Param(p,v) =>{
                if let Some(param) = old_sabun.remove(&def_key){
                    sabun.insert(def_key.to_string(), param);
                }
                new_map.insert(def_key,RootValue::Param(p,v));
            },
            RootValue::Mut(m) =>{
                if let Some(RootValue::Mut(old_m)) = old_def.remove(&def_key){
                    let new_m = adjust_mut_list(m, old_m, &Names::new(&def_key))?;
                    new_map.insert(def_key, RootValue::Mut(new_m));
                } else{
                    new_map.insert(def_key, RootValue::Mut(m));
                }
            },
            _ =>{
                //MutとParam以外にadjustする対象はないはず
                new_map.insert(def_key, def_value);
            },
        }

    }
    let new = RootObject::new(new_map, sabun, old_hash);

    if validation{
        validate_root(&new, true)?
    }
    return Ok(new);
}

