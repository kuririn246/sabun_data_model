use crate::structs::ref_value::RefValue;
use std::collections::{BTreeMap};
use crate::structs::qv::Qv;
use crate::indexmap::IndexMap;

///old_defとはrenameされている必要アリ oldは必要なし
/// auto_idとはいえdefaultは透過でいいと思うんだよねえ
pub fn adjust_auto_id_items_ref(renamed : &BTreeMap<String, String>,
                  new_def : &IndexMap<String, RefValue>,
                  old_def : &IndexMap<String, RefValue>, old : &IndexMap<String, RefValue>) -> IndexMap<String, RefValue> {
    let mut new: IndexMap<String, RefValue> = IndexMap::new();

    //undefableである場合、oldで定義されていないものであればundefinedにする
    for (key, value) in new_def {
        if value.value_type.is_undefable() {
            if old_def.get(key).is_none() {
                let val = RefValue::new(Qv::Undefined, value.value_type);
                //defaultがundefinedなのでsabunは定義しなくて良い
                new.insert(key.to_string(), val);
            }
        }
    }

    for (key, value) in old {
        let key = renamed.get(key).unwrap_or(key);
        let default_value = &value.default_value;

        if let Some(new_def_val) = new_def.get(key) {
            let mut val = RefValue::new(default_value.clone(), new_def_val.value_type.clone());
            if let Some(sabun) = &value.sabun_value {
                val.sabun_value = Some(sabun.clone());
            }
            new.insert(key.to_string(), val);
        }
    }

    return new;
}