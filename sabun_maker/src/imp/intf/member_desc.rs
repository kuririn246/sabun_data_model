use crate::imp::structs::value_type::VarType;
use crate::imp::structs::rust_value::RustMemberType;
use crate::imp::structs::list_value::ListDefValue;
use crate::imp::structs::list_def_obj::ListDefObj;

#[derive(Debug, PartialEq, Clone)]
pub struct MemberDesc{
    name : String,
    value_type : VarType,
    member_type : RustMemberType,
    is_old : bool,
}

impl MemberDesc{
    pub(crate) fn new(name : String, value_type : VarType, member_type : RustMemberType, is_old : bool) -> MemberDesc{
        MemberDesc{ name, value_type, member_type, is_old }
    }

    pub fn name(&self) -> &str{ &self.name }
    pub fn var_type(&self) -> &VarType { &self.value_type }
    pub fn member_type(&self) -> &RustMemberType { &self.member_type }
    pub fn is_old(&self) -> bool{ self.is_old }
}

#[derive(Debug, PartialEq, Clone)]
pub struct MemberDescs{
    items : Vec<MemberDesc>
}

impl MemberDescs{
    pub(crate) fn new(items : Vec<MemberDesc>) -> MemberDescs{ MemberDescs{ items }}
    pub fn items(&self) -> &[MemberDesc]{ &self.items }
}

pub fn get_list_def_desc(def : &ListDefObj) -> MemberDescs{
    let mut vec : Vec<MemberDesc> = Vec::with_capacity(def.default().len());
    for (k,val) in def.default(){
        let mem = k.to_string();
        let is_old = def.old().contains(k);
        let (mt, vt) = match val{
            ListDefValue::Param(p, vt) =>{
                (p.type_num(), vt.clone())
            },
            ListDefValue::InnerDataDef(_) => (RustMemberType::InnerData, VarType::Normal),
            ListDefValue::InnerListDef(_) => (RustMemberType::InnerList, VarType::Normal),
            ListDefValue::InnerMutDef(d) => (RustMemberType::InnerMut,
                                             if d.undefinable(){ VarType::Undefiable } else{ VarType::Normal }),
        };
        vec.push(MemberDesc::new(mem, vt, mt, is_old));
    }
    MemberDescs::new(vec)
}

