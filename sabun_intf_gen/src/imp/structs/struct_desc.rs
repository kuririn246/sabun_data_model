use sabun_maker::intf::member_desc::{MemberDesc, KeyItem};
use sabun_maker::structs::{VarType, RustMemberType};

pub struct StructDesc{
    pub(crate) col_mod_name : String,
    pub(crate) item_mod_name : String,
    pub(crate) col_struct_name : String,
    pub(crate) item_struct_name : String,
    pub(crate) col_type : ColType,
    pub(crate) ptr_type : String,
    pub(crate) param_descs : Vec<ParamDesc>,
    pub(crate) is_mut : bool,
    pub(crate) keys : Option<Vec<KeyItem>>,
    pub(crate) col_undefiable : bool
}

pub enum ColType{ Data, List }

pub struct ParamDesc{
    pub(crate) is_old : bool,
    pub(crate) name : String,
    pub(crate) var_type : VarType,
    pub(crate) param_type : ParamType
}
impl ParamDesc {
    pub fn new(mem: &MemberDesc) -> ParamDesc {

    }
}
pub enum ParamType{
    Bool, Num, Str, NumArray, StrArray, NumArray2
}
impl ParamType{
    pub fn new(t : &RustMemberType){
        RustMemberType::
    }
}