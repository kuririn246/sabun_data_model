use crate::imp::structs::value_type::ValueType;
use crate::imp::structs::rust_value::RustValueType;
use crate::imp::structs::list_value::ListDefValue;
use crate::imp::structs::list_def_obj::ListDefObj;

#[derive(Debug, PartialEq, Clone)]
pub struct MemberDesc{
    name : String,
    value_type : ValueType,
    member_type : RustValueType,
    is_old : bool,
}

impl MemberDesc{
    pub(crate) fn new(name : String, value_type : ValueType, member_type : RustValueType, is_old : bool) -> MemberDesc{
        MemberDesc{ name, value_type, member_type, is_old }
    }

    pub fn name(&self) -> &str{ &self.name }
    pub fn value_type(&self) -> &ValueType{ &self.value_type }
    pub fn member_type(&self) -> &RustValueType{ &self.member_type }
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
            ListDefValue::InnerDataDef(_) => (RustValueType::InnerData, ValueType::Normal),
            ListDefValue::InnerListDef(_) => (RustValueType::InnerList, ValueType::Normal),
            ListDefValue::InnerMutDef(d) => (RustValueType::InnerMut,
                                             if d.undefinable(){ ValueType::Undefiable } else{ ValueType::Normal }),
        };
        vec.push(MemberDesc::new(mem, vt, mt, is_old));
    }
    MemberDescs::new(vec)
}

