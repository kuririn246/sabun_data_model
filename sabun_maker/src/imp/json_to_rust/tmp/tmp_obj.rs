use crate::structs::rust_value::{RustValue, RootValue, ListSabValue, ListDefValue};
use crate::structs::ref_value::{RefValue, RefSabValue};
use std::collections::{HashSet, HashMap};
use crate::structs::rust_list::{ListItem, MutListItem};
use json5_parser::Span;
use crate::error::Result;
use crate::structs::root_object::{RefDefObj, RootObject, ListDefObj};

pub(crate) struct TmpObj{
    pub default : HashMap<String, RustValue>,
    pub id : Option<IdValue>,
    pub include : Vec<String>,
    pub refs: TmpRefs,
    pub old : HashSet<String>,
    pub span : Span,
}

pub(crate) struct TmpRefs{
    pub map : HashMap<String, RefValue>,
    pub old : HashSet<String>,
    pub is_enum : bool,
    pub span : Span,
}

impl TmpRefs{
    pub fn new(capacity : usize, span : Span) -> TmpRefs{
        TmpRefs{ map : HashMap::with_capacity(capacity), old : HashSet::new(), is_enum : false, span }
    }

    // pub fn get_hash_map(self) -> HashMap<String, RefValue>{
    //     self.map.into_iter().collect()
    // }

    pub fn to_ref_def_obj(self) -> RefDefObj{
        RefDefObj::new(self.map,  self.is_enum, self.old)
    }
}


pub enum IdValue{
    Str(String),
    Num(u64)
}

impl TmpObj{
    pub fn new(capacity : usize, span : Span) -> TmpObj{
        TmpObj{ default : HashMap::with_capacity(capacity), id : None, include : vec![], refs : TmpRefs::new(0,span.clone()), old : HashSet::new(), span }
    }

    pub fn to_root_obj(self) -> Result<RootObject>{
        fn to_root_hash(map : HashMap<String, RustValue>) -> Result<HashMap<String, RootValue>>{
            let mut result : HashMap<String, RootValue> = HashMap::with_capacity(map.len());

            for (key,value) in map{
                match value.to_root_value(){
                    Ok(v) =>{ result.insert(key, v); },
                    Err(type_s) => Err(format!("{} root object can't have {}", key, type_s))?,
                }
            }
            Ok(result)
        }

        if self.id.is_some(){
            Err(format!("ID is not needed for the root obj"))?
        }
        if self.refs.map.len() != 0{
            Err(format!("Ref is not needed for the root obj"))?
        }

        Ok(RootObject::new(
            to_root_hash(self.default)?,
            HashMap::new(), self.old))
    }

    pub fn to_list_def_obj(self) -> Result<ListDefObj>{
        if self.id.is_some(){
            Err(format!("{} list def can't have ID {}", self.span.line_str(), self.span.slice()))?
        }
        Ok(ListDefObj::new(to_list_def_val_map(self.default, &self.span)?,
        self.refs.to_ref_def_obj(), self.old))
    }

    pub fn insert_default(&mut self, s : String, v : RustValue){
        self.default.insert(s, v);
    }

    pub fn to_list_item(self) -> Result<ListItem>{

        if self.id.is_some(){
            Err(format!("{} ID is not needed for a list item {}", self.span.line_str(), self.span.slice()))?
        }
        if self.old.len() != 0{
            Err(format!("{} Old is not needed for a list item {}", self.span.line_str(), self.span.slice()))?
        }
        if self.refs.old.len() != 0{
            Err(format!("{} Old is not needed for a list item {}", self.refs.span.line_str(), self.refs.span.slice()))?
        }

        Ok(ListItem::new(to_list_sab_map(self.default, &self.span)?, to_ref_sab_map(self.refs.map)))
    }

    pub fn to_list_item_with_id(self) -> Result<(String, ListItem)>{
        if self.id.is_none(){
            Err(format!("{} ID must be defined {}", self.span.line_str(), self.span.slice()))?
        }
        if self.old.len() != 0{
            Err(format!("{} Old is not needed for a list item {}", self.span.line_str(), self.span.slice()))?
        }
        if self.refs.old.len() != 0{
            Err(format!("{} Old is not needed for a list item {}", self.refs.span.line_str(), self.refs.span.slice()))?
        }
        match self.id.unwrap(){
            IdValue::Str(s) =>{
                Ok((s, ListItem::new(to_list_sab_map(self.default, &self.span)?,to_ref_sab_map(self.refs.map))))
            },
            IdValue::Num(_) =>{
                Err(format!("{} ID must be a string {}", self.span.line_str(), self.span.slice()))?
            }
        }
    }

    pub fn to_violated_list_item(self, id : usize) -> Result<MutListItem>{
        let id = match self.id {
            Some(IdValue::Num(id)) => id,
            Some(_) =>{
                Err(format!("{} Violated List's item's ID must be a number {}", self.span.line_str(), self.span.slice()))?
            },
            None => id as u64,
        };

        if self.old.len() != 0{
            Err(format!("{} Old is not needed for a violated list item {}", self.span.line_str(), self.span.slice()))?
        }
        if self.refs.old.len() != 0{
            Err(format!("{} Old is not needed for a violated list item {}", self.refs.span.line_str(), self.refs.span.slice()))?
        }

        Ok(MutListItem::new(id, to_list_sab_map(self.default, &self.span)?,to_ref_sab_map(self.refs.map)))
    }
}

fn to_list_sab_map(map : HashMap<String, RustValue>, span : &Span) -> Result<HashMap<String, ListSabValue>>{
    let mut result : HashMap<String, ListSabValue> = HashMap::with_capacity(map.len());
    for (k,v) in map{
        let sab = match v.to_list_sab_value(){
            Ok(a) => a,
            Err(s) =>{
                Err(format!("{} {} list items can't have {}", span.line_str(), k, s))?
            }
        };
        result.insert(k, sab);
    }
    Ok(result)
}

fn to_list_def_val_map(map : HashMap<String, RustValue>, span : &Span) -> Result<HashMap<String, ListDefValue>>{
    let mut result : HashMap<String, ListDefValue> = HashMap::with_capacity(map.len());
    for (k,v) in map{
        let sab = match v.to_list_def_value(){
            Ok(a) => a,
            Err(s) =>{
                Err(format!("{} {} list def can't have {}", span.line_str(), k, s))?
            }
        };
        result.insert(k, sab);
    }
    Ok(result)
}

fn to_ref_sab_map(map : HashMap<String, RefValue>) -> HashMap<String, RefSabValue>{
    let mut result : HashMap<String, RefSabValue> = HashMap::with_capacity(map.len());
    for(k,v) in map{
        result.insert(k, v.to_sab_value());
    }
    return result;
}



