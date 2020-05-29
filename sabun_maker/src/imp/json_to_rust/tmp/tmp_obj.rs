use crate::structs::rust_value::RustValue;
use crate::indexmap::IndexMap;
use crate::structs::ref_value::RefValue;
use std::collections::{HashSet, HashMap};
use crate::structs::rust_list::ListItem;
use json5_parser::Span;
use crate::error::Result;

pub struct TmpObj{
    pub default : IndexMap<String, RustValue>,
    pub id : Option<IdValue>,
    pub include : Vec<String>,
    pub refs: TmpRefs,
    pub old : HashSet<String>,
    pub span : Span,
}

pub struct TmpRefs{
    pub map : IndexMap<String, RefValue>,
    pub old : HashSet<String>,
    pub span : Span,
}

impl TmpRefs{
    pub fn get_hash_map(self) -> HashMap<String, RefValue>{
        self.map.into_iter().collect()
    }
}


pub enum IdValue{
    Str(String),
    Num(f64)
}

impl TmpObj{
    pub fn new(span : Span) -> TmpObj{
        TmpObj{ default : IndexMap::new(), id : None, include : vec![], refs : TmpRefs{ map : IndexMap::new(), old : HashSet::new(), span : span.clone() }, old : HashSet::new(), span }
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

        Ok(ListItem{ refs : self.refs.get_hash_map(), values : self.default.into_iter().collect() })
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
                Ok((s, ListItem{ refs : self.refs.get_hash_map(), values : self.default.into_iter().collect() }))
            },
            IdValue::Num(_) =>{
                Err(format!("{} ID must be a string {}", self.span.line_str(), self.span.slice()))?
            }
        }
    }
}