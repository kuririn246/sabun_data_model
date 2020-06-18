use crate::imp::structs::value_type::ValueType;
use crate::imp::structs::def_obj::RefDefObj;

impl RefDesc{
    pub(crate) fn new(name : String, value_type : ValueType, is_old : bool) -> RefDesc{
        RefDesc{ name, value_type, is_old }
    }
    pub fn name(&self) -> &str{ &self.name }
    pub fn value_type(&self) -> &ValueType{ &self.value_type }
    pub fn is_old(&self) -> bool{ self.is_old }
}

#[derive(Debug, PartialEq, Clone)]
pub struct RefDescs{
    is_enum : bool,
    items : Vec<RefDesc>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct RefDesc{
    name : String,
    value_type : ValueType,
    is_old : bool,
}

impl RefDescs{
    pub(crate) fn new(is_enum : bool, items : Vec<RefDesc>) -> RefDescs{ RefDescs{ is_enum, items } }
    pub fn is_enum(&self) -> bool{ self.is_enum }
    pub fn items(&self) -> &[RefDesc]{ &self.items }
}

pub fn get_ref_def_desc(def : *const RefDefObj) -> RefDescs{
    let def = unsafe{ def.as_ref().unwrap() };
    let mut vec : Vec<RefDesc> = Vec::with_capacity(def.refs().len());
    for (k,val) in def.refs(){
        let mem = k.to_string();
        let is_old = def.old().contains(k);
        let vt = val.value_type();
        vec.push(RefDesc::new(mem, vt,  is_old));
    }
    RefDescs::new(def.is_enum(), vec)
}