use std::collections::BTreeMap;
use crate::structs::rust_value::RustValue;
use crate::error::Result;
use crate::indexmap::IndexMap;
use crate::structs::qv::Qv;

///new_sabunを返す
pub fn adjust_list_item_values(
    renamed : &BTreeMap<String, String>,
    new_list_def : &IndexMap<String, RustValue>,
    new_def : &mut IndexMap<String, RustValue>,
    old_list_def : &IndexMap<String, RustValue>,
    old_def : IndexMap<String, RustValue>,
    old_sabun : IndexMap<String, RustValue>) -> Result<IndexMap<String, RustValue>>{



    // //undefiableである場合、oldで定義されていないものであればundefinedにする
    // for (key, value) in new_list_def {
    //     if value.value_type.is_undefiable() {
    //         if old_def.get(key).is_none() {
    //             if let Some(val) = new.get_mut(key) {
    //                 val.sabun_value = Some(Qv::Undefined);
    //             } else {
    //                 //List Itemの方では書かれていないRefのメンバ、つまりデフォルトのままになっているものはデータもないのだが、
    //                 //Undefinedを入れるためにはRefValueをnewする必要がある。その場合のdefault_valueはやはりundefinedということになるだろう。
    //                 //ここにundefinedが入っていることで、これはもともとのjsonでは定義されていなかったんだな、と分かるというメリットは有る
    //                 //jsonになかったundefinedと、jsonにあったundefinedで内部表現にだいぶ違いがあるというのは、
    //                 //直感的にわかりにくいかもしれない・・・？
    //                 let val = RefValue::new(Qv::Undefined, value.value_type);
    //                 //defaultがundefinedなのでsabunは定義しなくて良い
    //                 new.insert(key.to_string(), val);
    //             }
    //         }
    //     }
    // }
    //
    // if let Some(old) = old {
    //     for (key, value) in old {
    //         //oldで元々のjson値から書き換えられているデータは、newの対応するメンバの値も書き換える。
    //         if let Some(sabun) = value.sabun_value {
    //             let key = renamed.get(&key).unwrap_or(&key);
    //
    //             if let Some(val) = new.get_mut(key) {
    //                 val.sabun_value = Some(sabun);
    //             } else {
    //                 if let Some(new_def_val) = new_def.get(key) {
    //                     let mut val = RefValue::new(Qv::Undefined, new_def_val.value_type.clone());
    //                     val.sabun_value = Some(sabun);
    //                     new.insert(key.to_string(), val);
    //                 }
    //             }
    //         }
    //     }
    // }
    //
    // //len == 0 のときは allocateされないらしいので助かる
    // if new.len() == 0 {
    //     return Ok(None);
    // } else {
    //     return Ok(Some(new));
    // }

    todo!();
    //return Ok(new);

}