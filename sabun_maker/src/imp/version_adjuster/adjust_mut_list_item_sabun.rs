use crate::structs::root_object::ListDefObj;
use crate::structs::rust_value::{RustValue};
use std::collections::HashMap;
use crate::error::Result;
use crate::imp::json_to_rust::names::Names;
use crate::imp::version_adjuster::adjust_mut_list::adjust_inner_mut_list;

pub fn adjust_mut_list_item_sabun(def : &ListDefObj, old_sabun : HashMap<String, RustValue>, names : &Names) -> Result<HashMap<String, RustValue>>{
    let mut old_sabun = old_sabun;

    //最大量で見積もっておく。デフォルトから変化しない場合はsabunには加わらないが、sabun.len()だと、
    //undefinedで一個増えただけでテーブル再構成＆cap2倍にされてしまう
    let mut result : HashMap<String, RustValue> = HashMap::with_capacity(def.default().len());

    for (def_key, def_v) in def.default(){
        let sabun_v = if let Some(v) = old_sabun.remove(def_key){ v } else {
            match def_v{
                RustValue::Param(p, vt) =>{
                    if vt.undefiable(){
                        RustValue::Param(p.to_undefined(), vt.clone())
                    } else{
                        continue;
                    }
                },
                RustValue::InnerMutDef(mut_def) =>{
                    if mut_def.undefinable(){
                        RustValue::InnerMut(None)
                    } else{
                        continue;
                    }
                },
                _ =>{
                    Err(format!("{} {} mut list's default item can only have Param or InnerMutDef", names, def_key))?
                }
            }
        };
        match sabun_v{
            RustValue::Param(p, v) =>{
                //型チェックめんどいからなしでいいかな・・・？
                result.insert(def_key.to_string(), RustValue::Param(p, v));
            },
            RustValue::InnerMut(op) =>{
                match op{
                    Some(im) =>{
                        let new = adjust_inner_mut_list(def, im, &names.append(def_key))?;
                        result.insert(def_key.to_string(), RustValue::InnerMut(Some(new)));
                    },
                    None =>{
                        result.insert(def_key.to_string(), RustValue::InnerMut(None));
                    }
                }
            },
            _ =>{ Err(format!("{} {} mut list items can only have Param or InnerMut", names, def_key))? }
        }
    }
    //最大量で見積もってshrink_to_fitするというのは安全で効率的に一見見えるが・・・所詮は事前最適化で効果は不明である。
    //performanceが向上する自信がないのでコメントアウトしておく
    //result.shrink_to_fit();
    Ok(result)
}