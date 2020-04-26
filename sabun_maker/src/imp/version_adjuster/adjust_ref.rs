use crate::structs::ref_value::RefValue;
//use std::collections::{BTreeMap, HashSet};
use crate::structs::qv::Qv;
use crate::error::Result;
use crate::indexmap::IndexMap;

pub fn adjust_ref(new_def : &IndexMap<String, RefValue>, new : Option<IndexMap<String, RefValue>>,
                  old_def : &IndexMap<String, RefValue>, old : Option<IndexMap<String, RefValue>>) -> Option<IndexMap<String, RefValue>> {
    let mut new = new.unwrap_or_else(|| IndexMap::new());

    //undefableである場合、oldで定義されていないものであればundefinedにする
    for (key, value) in new_def {
        if value.value_type.is_undefable() {
            if old_def.get(key).is_none() {
                if let Some(val) = new.get_mut(key) {
                    val.sabun_value = Some(Qv::Undefined);
                } else {
                    //List Itemの方では書かれていないRefのメンバ、つまりデフォルトのままになっているものはデータもないのだが、
                    //Undefinedを入れるためにはRefValueをnewする必要がある。その場合のdefault_valueはやはりundefinedということになるだろう。
                    //ここにundefinedが入っていることで、これはもともとのjsonでは定義されていなかったんだな、と分かるというメリットは有る
                    //jsonになかったundefinedと、jsonにあったundefinedで内部表現にだいぶ違いがあるというのは、
                    //直感的にわかりにくいかもしれない・・・？
                    let val = RefValue::new(Qv::Undefined, value.value_type);
                    //defaultがundefinedなのでsabunは定義しなくて良い
                    new.insert(key.to_string(), val);
                }
            }
        }
    }

    if let Some(old) = old {
        for (key, value) in old {
            //oldで元々のjson値から書き換えられているデータは、newの対応するメンバの値も書き換える。
            if let Some(sabun) = value.sabun_value {

                if let Some(val) = new.get_mut(&key) {
                    val.sabun_value = Some(sabun);
                } else {
                    if let Some(new_def_val) = new_def.get(&key) {
                        let mut val = RefValue::new(Qv::Undefined, new_def_val.value_type.clone());
                        val.sabun_value = Some(sabun);
                        new.insert(key, val);
                    }
                }
            }
        }
    }

    //len == 0 のときは allocateされないらしいので助かる
    if new.len() == 0 {
        return None;
    } else {
        return Some(new);
    }
}

