use crate::imp::json_to_rust::tmp::tmp_obj::{ IdValue};
use std::collections::{HashSet, HashMap, BTreeSet, BTreeMap};
use crate::imp::structs::root_object::ListDefObj;
use crate::imp::structs::rust_value::RustValue;
use crate::imp::structs::rust_list::{ListItem, MutListItem, ConstData, ConstList, MutList, InnerData, InnerList, InnerMutList};
use crate::imp::structs::ref_value::RefValue;

pub(crate) struct TmpJsonList{
    pub vec : Vec<TmpJsonObj>,
    pub old : Option<BTreeSet<String>>,
    pub default : Option<ListDefObj>,
    pub compatible : Option<BTreeSet<String>>,
    pub next_id : Option<u64>,
}

pub(crate) struct TmpJsonObj{
    pub default : BTreeMap<String, RustValue>,
    pub id : Option<IdValue>,
    pub refs: Option<TmpJsonRefs>,
    pub old : Option<BTreeSet<String>>,
}

impl TmpJsonObj{
    pub fn from_list_item(l : &ListItem, id : Option<&String>) -> TmpJsonObj{
        let value_map : HashMap<String, RustValue> = l.values().iter().map(|(k,v)| (k.to_string(), v.clone().into_rust_value_for_json())).collect();
        let ref_map : HashMap<String, RefValue> = l.refs().iter().map(|(k,v)| (k.to_string(), v.clone().into_ref_value_for_json())).collect();
        TmpJsonObj{ default : btree_map(&value_map),
            refs : TmpJsonRefs::from_list_item(&ref_map),
            id : id.map(|s| IdValue::Str(s.to_string())), old : None }
    }

    pub fn from_mut_list_item(l : &MutListItem) -> TmpJsonObj{
        let value_map : HashMap<String, RustValue> = l.values().iter().map(|(k,v)| (k.to_string(), v.clone().into_rust_value_for_json())).collect();
        let ref_map : HashMap<String, RefValue> = l.refs().iter().map(|(k,v)| (k.to_string(), v.clone().into_ref_value_for_json())).collect();
        TmpJsonObj{
            default : btree_map(&value_map),
            refs : TmpJsonRefs::from_list_item(&ref_map),
            id : Some(IdValue::Num(l.id())), old : None }
    }
}

pub(crate) struct TmpJsonRefs{
    pub map : BTreeMap<String, RefValue>,
    pub old : Option<BTreeSet<String>>,
    pub is_enum : bool,
}

impl TmpJsonRefs{
    pub fn from_map(map : &HashMap<String, RefValue>, old : Option<&HashSet<String>>, is_enum : bool) -> TmpJsonRefs{
        TmpJsonRefs{
            map : btree_map(map),
            old : old.map(|s| btree_set(s)), is_enum }
    }

    pub fn from_list_item(map : &HashMap<String, RefValue>) -> Option<TmpJsonRefs> {
        if map.len() != 0 {
            Some(TmpJsonRefs::from_map(map, None, false))
        } else { None }
    }
}

fn get_from_set(set : &HashSet<String>) -> Option<BTreeSet<String>>{
    if set.is_empty(){
        None
    } else{
        Some(btree_set(set))
    }
}

impl TmpJsonList{
    pub fn from_const_data(l : &ConstData) -> TmpJsonList{
        //最近のハッシュマップは中身の順番がランダム化されるようなので、btree_mapにうつして順番を揃える
         TmpJsonList{ vec: btree_map(l.list()).iter().map(|(id,item)| TmpJsonObj::from_list_item(item, Some(id))).collect(),
             compatible : None, next_id: None, old : get_from_set(l.old()), default : Some(l.default().clone()) }
    }

    pub fn from_const_list(l : &ConstList) -> TmpJsonList{
        TmpJsonList{ vec: l.list().iter().map(|item| TmpJsonObj::from_list_item(item, None)).collect(),
            compatible : None, next_id: None, old : None, default : Some(l.default().clone()) }
    }

    pub fn from_mut_list(l : &MutList) -> TmpJsonList{
        TmpJsonList{ vec: l.list().iter().map(|(_,item)| TmpJsonObj::from_mut_list_item(item)).collect(),
            compatible : get_from_set(l.compatible()), next_id: Some(l.next_id()), old : None, default : Some(l.default().clone()) }
    }

    pub fn from_inner_data(l : &InnerData) -> TmpJsonList{
        TmpJsonList{ vec : btree_map(l.list()).iter().map(|(id, item)| TmpJsonObj::from_list_item(item, Some(id))).collect(),
            compatible : None, next_id: None, old : get_from_set(l.old()), default : None }
    }

    pub fn from_inner_list(l : &InnerList) -> TmpJsonList{
        TmpJsonList{ vec : l.list().iter().map(|item| TmpJsonObj::from_list_item(item, None)).collect(),
            compatible : None, next_id: None, old : None, default : None }
    }

    pub fn from_inner_mut(l : &InnerMutList) -> TmpJsonList{
        TmpJsonList{ vec : l.list().iter().map(|(_id, item)| TmpJsonObj::from_mut_list_item(item)).collect(),
            compatible : None, next_id: None, old : None, default : None }
    }
}

pub fn btree_set(hash : &HashSet<String>) -> BTreeSet<String>{
    hash.iter().map(|s| s.to_string()).collect()
}

pub fn btree_map<T : Clone>(hash : &HashMap<String, T>) -> BTreeMap<String, T>{
    hash.iter().map(|(key,val)|(key.to_string(), val.clone())).collect()
}
