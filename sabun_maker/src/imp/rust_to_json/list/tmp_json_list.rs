use crate::structs::rust_list::{ConstList, ConstData, ListItem};
use crate::imp::json_to_rust::tmp::tmp_obj::{TmpObj, IdValue};
use std::collections::{HashSet, HashMap};
use crate::structs::root_object::ListDefObj;
use crate::structs::rust_value::RustValue;
use crate::structs::ref_value::RefValue;

pub struct TmpJsonList{
    pub vec : Vec<TmpJsonObj>,
    pub old : Option<HashSet<String>>,
    pub default : Option<ListDefObj>,
    pub compatible : Option<HashSet<String>>,
    pub next_id : Option<u64>,
}

pub struct TmpJsonObj{
    pub default : HashMap<String, RustValue>,
    pub id : Option<IdValue>,
    pub refs: TmpJsonRefs,
    pub old : Option<HashSet<String>>,
}

impl TmpJsonObj{
    pub fn from_list_item(l : &ListItem, id : Option<&String>) -> TmpJsonObj{
        TmpJsonObj{ default : l.values.as_ref().clone(), refs : TmpJsonRefs::from_map(&l.refs, None, false), id : id.map(|s| IdValue::Str(s.to_string())), old : None }
    }
}

pub struct TmpJsonRefs{
    pub map : HashMap<String, RefValue>,
    pub old : Option<HashSet<String>>,
    pub is_enum : bool,
}

impl TmpJsonRefs{
    pub fn from_map(map : &HashMap<String, RefValue>, old : Option<&HashSet<String>>, is_enum : bool) -> TmpJsonRefs{
        TmpJsonRefs{ map : map.clone(), old : old.map(|o| o.clone()), is_enum }
    }
}



impl TmpJsonList{
    // pub fn from_const_data(l : &ConstData) -> TmpJsonList{
    //     TmpJsonList{ vec: l.list, compatible : None, next_id: None, old : None, default : Some(l.default.as_ref().clone()) }
    // }

    pub fn from_const_list(l : &ConstList) -> TmpJsonList{
        TmpJsonList{ vec: l.list.iter().map(|item| TmpJsonObj::from_list_item(item, None)).collect(),
            compatible : None, next_id: None, old : None, default : Some(l.default.as_ref().clone()) }
    }


}