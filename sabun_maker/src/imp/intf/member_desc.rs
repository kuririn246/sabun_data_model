use crate::imp::structs::value_type::ValueType;
use crate::imp::structs::rust_value::RustValueType;
use crate::imp::structs::def_obj::{ListDefObj, RefDefObj};
use crate::imp::structs::list_value::ListDefValue;

pub struct MemberDesc{
    pub member : String,
    pub value_type : ValueType,
    pub member_type : RustValueType,
    pub is_old : bool,
}

impl MemberDesc{
    pub fn new(member : String, value_type : ValueType, member_type : RustValueType, is_old : bool) -> MemberDesc{
        MemberDesc{ member, value_type, member_type, is_old }
    }
}

pub struct RefDesc{
    pub member : String,
    pub value_type : ValueType,
    pub is_old : bool,
}

impl RefDesc{
    pub fn new(member : String, value_type : ValueType, is_old : bool) -> RefDesc{
        RefDesc{ member, value_type, is_old }
    }
}

pub fn get_list_def_desc(def : &ListDefObj) -> Vec<MemberDesc>{
    let mut vec : Vec<MemberDesc> = Vec::with_capacity(def.default().len());
    for (k,val) in def.default(){
        let mem = k.to_string();
        let is_old = def.old().contains(k);
        let (mt, vt) = match val{
            ListDefValue::Param(p, vt) =>{
                (p.type_num(), vt.clone())
            },
            ListDefValue::InnerDataDef(_) => (RustValueType::InnerData, ValueType::Normal),
            ListDefValue::InnerListDef(_) => (RustValueType::InnerList, ValueType::Normal),
            ListDefValue::InnerMutDef(d) => (RustValueType::InnerMut,
                                             if d.undefinable(){ ValueType::Undefiable } else{ ValueType::Normal }),
        };
        vec.push(MemberDesc::new(mem, vt, mt, is_old));
    }
    vec
}

pub fn get_ref_def_desc(def : &RefDefObj) -> Vec<RefDesc>{
    let mut vec : Vec<RefDesc> = Vec::with_capacity(def.refs().len());
    for (k,val) in def.refs(){
        let mem = k.to_string();
        let is_old = def.old().contains(k);
        let vt = val.value_type();
        vec.push(RefDesc::new(mem, vt,  is_old));
    }
    vec
}