use crate::imp::json_to_rust::tmp::tmp_obj::TmpObj;
use std::collections::HashSet;
use crate::structs::root_object::ListDefObj;
use crate::structs::rust_list::{ConstList, ListItem, ConstData, MutList, MutListItem, InnerList, InnerData, InnerMutList};
use json5_parser::Span;
use crate::error::Result;
use crate::indexmap::str_vec_map::StrVecMap;
use linked_hash_map::LinkedHashMap;

pub struct TmpList{
    pub vec : Vec<TmpObj>,
    ///複数回定義のエラーを検出したいのでOptionにする
    pub old : Option<HashSet<String>>,
    pub default : Option<ListDefObj>,
    pub compatible : Option<HashSet<String>>,
    pub next_id : Option<u64>,
    pub span : Span,
}

impl TmpList{
    pub fn new(capacity : usize, span : Span) -> TmpList{
        TmpList{ vec : Vec::with_capacity(capacity), old : None, default : None, compatible : None, next_id : None, span }
    }

    pub fn to_const_list(self) -> Result<ConstList>{
        if self.compatible.is_some(){
            Err(format!("{} Compatible is not needed for a List {}", self.span.line_str(), self.span.slice()))?
        }
        if self.old.is_some(){
            Err(format!("{} Old is not needed for a List {}", self.span.line_str(), self.span.slice()))?
        }
        if self.default.is_none(){
            Err(format!("{} Default must be defined {}", self.span.line_str(), self.span.slice()))?
        }
        if self.next_id.is_some(){
            Err(format!("{} NextID must not be defined {}", self.span.line_str(), self.span.slice()))?
        }

        Ok(ConstList{ default : self.default.unwrap(), list : to_list_items(self.vec)? })
    }

    pub fn to_inner_list(self) -> Result<InnerList>{
        if self.compatible.is_some(){
            Err(format!("{} Compatible is not needed for InnerList {}", self.span.line_str(), self.span.slice()))?
        }
        if self.old.is_some(){
            Err(format!("{} Old is not needed for InnerList {}", self.span.line_str(), self.span.slice()))?
        }
        if self.default.is_some(){
            Err(format!("{} Default must not be defined for InnerList {}", self.span.line_str(), self.span.slice()))?
        }
        if self.next_id.is_some(){
            Err(format!("{} NextID must not be defined for InnerList {}", self.span.line_str(), self.span.slice()))?
        }
        let compatible = self.compatible.unwrap_or_else(|| HashSet::new());

        Ok(InnerList{ compatible, list : to_list_items(self.vec)? })
    }

    pub fn to_const_data(self) -> Result<ConstData>{
        if self.compatible.is_some(){
            Err(format!("{} Compatible is not needed for Data {}", self.span.line_str(), self.span.slice()))?
        }
        if self.default.is_none(){
            Err(format!("{} Default must be defined {}", self.span.line_str(), self.span.slice()))?
        }
        if self.next_id.is_some(){
            Err(format!("{} NextID must not be defined {}", self.span.line_str(), self.span.slice()))?
        }
        let old = self.old.unwrap_or_else(|| HashSet::new());

        Ok(ConstData{ default : self.default.unwrap(), old, list : to_data_items(self.vec)? })
    }
    pub fn to_inner_data(self) -> Result<InnerData>{
        if self.compatible.is_some(){
            Err(format!("{} Compatible is not needed for Data {}", self.span.line_str(), self.span.slice()))?
        }
        if self.default.is_some(){
            Err(format!("{} Default must not be defined {}", self.span.line_str(), self.span.slice()))?
        }
        if self.next_id.is_some(){
            Err(format!("{} NextID must not be defined {}", self.span.line_str(), self.span.slice()))?
        }
        let old = self.old.unwrap_or_else(|| HashSet::new());

        Ok(InnerData{ old, list : to_data_items(self.vec)? })
    }

    pub fn to_mut_list(self) -> Result<MutList>{
        if self.old.is_some(){
            Err(format!("{} Old is not needed for MutList {}", self.span.line_str(), self.span.slice()))?
        }
        if self.default.is_none(){
            Err(format!("{} Default must be defined {}", self.span.line_str(), self.span.slice()))?
        }
        if self.next_id.is_some(){
            Err(format!("{} NextID is not needed for MutList {}", self.span.line_str(), self.span.slice()))?
        }
        if self.vec.len() != 0{
            Err(format!("{} MutList must not have items {}", self.span.line_str(), self.span.slice()))?
        }
        let compatible = self.compatible.unwrap_or_else(|| HashSet::new());
        Ok(MutList{ default : self.default.unwrap(), compatible, list : LinkedHashMap::new(), next_id : 0 })
    }

    pub fn to_inner_mut_list(self) -> Result<InnerMutList>{

        if self.old.is_some(){
            Err(format!("{} Old is not needed for InnerMutList {}", self.span.line_str(), self.span.slice()))?
        }
        if self.default.is_some(){
            Err(format!("{} Default must not be defined {}", self.span.line_str(), self.span.slice()))?
        }
        if self.next_id.is_some(){
            Err(format!("{} NextID is not needed for InnerMutList {}", self.span.line_str(), self.span.slice()))?
        }
        if self.vec.len() != 0{
            Err(format!("{} InnerMutList must not have items {}", self.span.line_str(), self.span.slice()))?
        }
        let compatible = self.compatible.unwrap_or_else(|| HashSet::new());
        Ok(InnerMutList{ list : LinkedHashMap::new(), compatible, next_id : 0 })
    }

    ///MutListは中身があってはいけないのだが、そのルールを破壊する裏道が用意されている。
    pub fn to_violated_list(self) -> Result<MutList>{
        if self.old.is_some(){
            Err(format!("{} Old is not needed for ViolatedList {}", self.span.line_str(), self.span.slice()))?
        }
        if self.default.is_none(){
            Err(format!("{} Default must be defined {}", self.span.line_str(), self.span.slice()))?
        }
        if self.next_id.is_none(){
            Err(format!("{} NextID is needed for ViolatedList {}", self.span.line_str(), self.span.slice()))?
        }
        let next_id = self.next_id.unwrap();
        let compatible = self.compatible.unwrap_or_else(|| HashSet::new());

        Ok(MutList{ default : self.default.unwrap(), list : to_violated_list_items(self.vec)?, compatible, next_id })
    }

    ///MutListは中身があってはいけないのだが、そのルールを破壊する裏道が用意されている。
    pub fn to_inner_violated_list(self) -> Result<InnerMutList>{
        if self.old.is_some(){
            Err(format!("{} Old is not needed for ViolatedList {}", self.span.line_str(), self.span.slice()))?
        }
        if self.default.is_some(){
            Err(format!("{} Default must not be defined {}", self.span.line_str(), self.span.slice()))?
        }
        if self.next_id.is_none(){
            Err(format!("{} NextID is needed for ViolatedList {}", self.span.line_str(), self.span.slice()))?
        }
        let next_id = self.next_id.unwrap();
        let compatible = self.compatible.unwrap_or_else(|| HashSet::new());

        Ok(InnerMutList{ list : to_violated_list_items(self.vec)?, compatible, next_id })
    }

    pub fn to_inner_def(self) -> Result<ListDefObj>{
        if self.compatible.is_some(){
            Err(format!("{} Compatible is not needed for InnerDef {}", self.span.line_str(), self.span.slice()))?
        }
        if self.old.is_some(){
            Err(format!("{} Old is not needed for InnerDef {}", self.span.line_str(), self.span.slice()))?
        }
        if self.default.is_none(){
            Err(format!("{} Default must be defined {}", self.span.line_str(), self.span.slice()))?
        }
        if self.next_id.is_some(){
            Err(format!("{} NextID is not needed for InnerDef {}", self.span.line_str(), self.span.slice()))?
        }
        if self.vec.len() != 0{
            Err(format!("{} InnerDef must not have items {}", self.span.line_str(), self.span.slice()))?
        }

        Ok(self.default.unwrap())
    }

}


fn to_list_items(vec : Vec<TmpObj>) -> Result<Vec<ListItem>>{
    let mut result : Vec<ListItem> = Vec::with_capacity(vec.len());
    for item in vec{
        result.push(item.to_list_item()?);
    }
    return Ok(result);
}

fn to_data_items(vec : Vec<TmpObj>) -> Result<StrVecMap<ListItem>>{
    let mut result : StrVecMap<ListItem> = StrVecMap::with_capacity(vec.len());
    for item in vec{
        let (s,m) = item.to_list_item_with_id()?;
        result.insert(s, m);
    }
    return Ok(result);
}

fn to_violated_list_items(vec : Vec<TmpObj>) -> Result<LinkedHashMap<u64, MutListItem>>{
    let mut result : LinkedHashMap<u64, MutListItem> = LinkedHashMap::with_capacity(vec.len());
    for item in vec{
        let item = item.to_violated_list_item()?;
        result.insert(item.id, item);
    }
    return Ok(result);
}