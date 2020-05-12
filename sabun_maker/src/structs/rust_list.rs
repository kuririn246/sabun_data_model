use crate::structs::list_type::ListType;
use crate::structs::rust_object::RustObject;
use std::collections::BTreeMap;
use linked_hash_map::LinkedHashMap;

#[derive(Debug, PartialEq)]
pub struct RustList{
    pub list_type : ListType,
    pub default : ListDef,
    pub list : LinkedHashMap<String, RustObject>,
    pub redef : BTreeMap<String, String>,
}

#[derive(Debug, PartialEq)]
pub enum ListDef{
    Def(RustObject),
    Rent(String),
    InnerList,
}

impl RustList{
    pub fn is_auto_id(&self) -> bool{
        match self.list_type{
            ListType::AutoID(_) => true,
            _ => false,
        }
    }

    pub fn is_null_auto_id(&self) -> bool{
        match self.list_type{
            ListType::AutoID(val) =>{
                val.is_none()
            },
            _ =>{ false }
        }
    }

    pub fn set_auto_id(&mut self, id : u64) -> Result<(), ()>{
        match self.list_type{
            ListType::AutoID(_) =>{
                self.list_type = ListType::AutoID(Some(id));
                Ok(())
            },
            _=>{ Err(()) }
        }
    }

    pub fn new() -> RustList{
        RustList{
            list_type : ListType::Normal,
            default : RustObject::new(),
            list : LinkedHashMap::new(),
            redef : BTreeMap::new(),
        }
    }
}