use sabun_maker::intf::member_desc::{KeyItem};
use sabun_maker::structs::{VarType};

#[derive(Debug, Clone, PartialEq)]
pub struct StructDesc{
    pub(crate) col_id : String,
    pub(crate) col_mod_name : String,
    pub(crate) item_mod_name : String,
    pub(crate) col_struct_name : String,
    pub(crate) item_struct_name : String,
    pub(crate) col_type : ColType,
    pub(crate) col_ptr_type : String,
    pub(crate) col_is_old : bool,
    pub(crate) item_ptr_type : String,
    pub(crate) params : Vec<ParamItem>,
    pub(crate) refs : Vec<RefItem>,
    pub(crate) ref_is_enum : bool,
    pub(crate) is_mut : bool,
    pub(crate) keys : Vec<KeyItem>,
    pub(crate) col_undefiable : bool,
    pub(crate) children : Vec<StructDesc>,
}

impl StructDesc{
    pub fn is_root(&self) -> bool{ self.col_id.is_empty() }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ColType{ Data, List, Root }

#[derive(Debug, Clone, PartialEq)]
pub struct ParamItem{
    pub(crate) is_old : bool,
    //pub(crate) snake_name : String,
    pub(crate) id : String,
    pub(crate) var_type : VarType,
    //pub(crate) param_type : ParamType
    pub(crate) value_type_name : String,
    pub(crate) value_type_nickname : String,
    pub(crate) is_ref : bool,
}

// #[derive(Debug, Clone, PartialEq)]
// pub enum ParamType{
//     Bool, Num, //Str, NumArray, StrArray, NumArray2
// }

#[derive(Debug, Clone, PartialEq)]
pub struct RefItem{
    pub(crate) is_old : bool,
    //pub(crate) snake_name: String,
    pub(crate) id : String,
    pub(crate) var_type : VarType,
}