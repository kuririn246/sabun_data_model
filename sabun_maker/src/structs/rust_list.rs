use crate::structs::list_type::ListType;
use crate::structs::root_object::{RustObject, ListDefObj};
use std::collections::BTreeMap;
use linked_hash_map::LinkedHashMap;
use std::rc::{Rc, Weak};
use crate::indexmap::IndexMap;
use crate::structs::rust_value::RustValue;
use crate::structs::ref_value::RefValue;

#[derive(Debug, PartialEq)]
pub struct ConstList{
    pub default : ListDef,
    pub list : IndexMap<String, ListItem>,
}

#[derive(Debug, PartialEq)]
pub struct MutList{
    pub default : ListDef,
    pub list : Vec<ListItem>
}

pub struct ListItem{
    pub values : IndexMap<String, RustValue>,
    pub refs : IndexMap<String, RefValue>,
}


#[derive(Debug, PartialEq)]
pub enum ListDef{
    Def(ListDefObj),
    InnerDef(Rc<ListDefObj>),
    InnerItem(Weak<ListDefObj>),
}
//
// impl RustList{
//     pub fn is_auto_id(&self) -> bool{
//         match self.list_type{
//             ListType::AutoID(_) => true,
//             _ => false,
//         }
//     }
//
//     pub fn is_null_auto_id(&self) -> bool{
//         match self.list_type{
//             ListType::AutoID(val) =>{
//                 val.is_none()
//             },
//             _ =>{ false }
//         }
//     }
//
//     pub fn set_auto_id(&mut self, id : u64) -> Result<(), ()>{
//         match self.list_type{
//             ListType::AutoID(_) =>{
//                 self.list_type = ListType::AutoID(Some(id));
//                 Ok(())
//             },
//             _=>{ Err(()) }
//         }
//     }
//
//     pub fn new() -> RustList{
//         RustList{
//             list_type : ListType::Normal,
//             default : ListDef::Def(RustObject::new()),
//             list : LinkedHashMap::new(),
//             redef : BTreeMap::new(),
//         }
//     }
// }