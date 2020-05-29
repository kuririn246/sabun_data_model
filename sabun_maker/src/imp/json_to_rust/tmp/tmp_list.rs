use crate::imp::json_to_rust::tmp::tmp_obj::TmpObj;
use std::collections::HashSet;
use crate::structs::root_object::ListDefObj;
use crate::structs::rust_list::{ConstList, ListItem, ConstData, MutList, MutListItem, InnerList};
use json5_parser::Span;
use crate::error::Result;
use crate::indexmap::IndexMap;

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
    pub fn new(span : Span) -> TmpList{
        TmpList{ vec : vec![], old : None, default : None, compatible : None, next_id : None, span }
    }

    pub fn to_const_list(self) -> Result<ConstList>{
        if self.old.is_some(){
            Err(format!("{} Old is not needed for a List {}", self.span.line_str(), self.span.slice()))?
        }
        if self.default.is_none(){
            Err(format!("{} Default must be defined {}", self.span.line_str(), self.span.slice()))?
        }
        let compatible = self.compatible.unwrap_or_else(|| HashSet::new());

        Ok(ConstList{ default : self.default.unwrap(), compatible, list : to_list_items(self.vec)? })
    }

    pub fn to_const_data(self) -> Result<ConstData>{
        if self.compatible.is_some(){
            Err(format!("{} Compatible is not needed for Data {}", self.span.line_str(), self.span.slice()))?
        }
        if self.default.is_none(){
            Err(format!("{} Default must be defined {}", self.span.line_str(), self.span.slice()))?
        }
        let old = self.old.unwrap_or_else(|| HashSet::new());

        Ok(ConstData{ default : self.default.unwrap(), old, list : to_data_items(self.vec)? })
    }

    pub fn to_mut_list(self) -> Result<MutList>{
        if self.compatible.is_some(){
            Err(format!("{} Compatible is not needed for MutList {}", self.span.line_str(), self.span.slice()))?
        }
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
        Ok(MutList{ default : self.default.unwrap(), list : vec![], next_id : 0 })
    }

    pub fn to_violated_list(self) -> Result<MutList>{
        if self.compatible.is_some(){
            Err(format!("{} Compatible is not needed for MutList {}", self.span.line_str(), self.span.slice()))?
        }
        if self.old.is_some(){
            Err(format!("{} Old is not needed for MutList {}", self.span.line_str(), self.span.slice()))?
        }
        if self.default.is_none(){
            Err(format!("{} Default must be defined {}", self.span.line_str(), self.span.slice()))?
        }
        if self.next_id.is_none(){
            Err(format!("{} NextID is needed for ViolatedList {}", self.span.line_str(), self.span.slice()))?
        }
        let next_id = self.next_id.unwrap();

        Ok(MutList{ default : self.default.unwrap(), list : to_mut_list_items(self.vec)?, next_id })
    }
    pub fn to_inner_list(self) -> Result<Inner>{

    }
}

pub enum Inner{
    Def(ListDefObj),
    List(InnerList),
}

fn to_list_items(vec : Vec<TmpObj>) -> Result<Vec<ListItem>>{
    let mut result : Vec<ListItem> = vec![];
    for item in vec{
        result.push(item.to_list_item()?);
    }
    return Ok(result);
}

fn to_data_items(vec : Vec<TmpObj>) -> Result<IndexMap<String, ListItem>>{
    let mut result : IndexMap<String, ListItem> = IndexMap::new();
    for item in vec{
        let (s,m) = item.to_list_item_with_id()?;
        result.insert(s, m);
    }
    return Ok(result);
}

fn to_mut_list_items(vec : Vec<TmpObj>) -> Result<Vec<MutListItem>>{
    let mut result : Vec<MutListItem> = vec![];
    for item in vec{

    }
}