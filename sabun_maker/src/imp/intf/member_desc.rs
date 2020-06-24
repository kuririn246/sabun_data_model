use crate::imp::structs::value_type::VarType;
use crate::imp::structs::rust_value::RustMemberType;
use crate::imp::structs::list_value::ListDefValue;
use crate::imp::structs::list_def_obj::ListDefObj;
use crate::imp::structs::root_obj::RootObject;
use crate::imp::structs::root_value::RootValue;
use crate::imp::intf::data::{get_kvs, DataKVs};

#[derive(Debug, PartialEq, Clone)]
pub struct MemberDesc{
    name : String,
    var_type: VarType,
    member_type : RustMemberType,
    is_old : bool,
    child_descs : Option<MemberDescs>,
}
impl MemberDesc{
    pub(crate) fn new(name : String, var_type : VarType, member_type : RustMemberType, is_old : bool, child_descs : Option<MemberDescs>) -> MemberDesc{
        MemberDesc{ name, var_type, member_type, is_old, child_descs }
    }

    pub fn name(&self) -> &str{ &self.name }
    pub fn var_type(&self) -> &VarType { &self.var_type }
    pub fn member_type(&self) -> &RustMemberType { &self.member_type }
    pub fn is_old(&self) -> bool{ self.is_old }
    pub fn child_descs(&self) -> Option<&MemberDescs>{ self.child_descs.as_ref() }
}

#[derive(Debug, PartialEq, Clone)]
pub struct MemberDescs{
    items : Vec<MemberDesc>,
    keys : Option<Vec<KeyItem>>,
}
impl MemberDescs{
    pub(crate) fn new(items : Vec<MemberDesc>) -> MemberDescs{ MemberDescs{ items, keys : None }}
    pub(crate) fn with_keys(items : Vec<MemberDesc>, keys : Vec<KeyItem>) -> MemberDescs{ MemberDescs{ items, keys : Some(keys) }}
    pub fn items(&self) -> &[MemberDesc]{ &self.items }
    pub fn keys(&self) -> Option<&[KeyItem]>{ self.keys.as_ref().map(|a| a.as_slice()) }
}

#[derive(Debug, PartialEq, Clone)]
pub struct KeyItem{
    key : String,
    is_old : bool,
}
impl KeyItem{
    pub(crate) fn new(key : String, is_old : bool) -> KeyItem{ KeyItem{ key, is_old }}
    pub fn key(&self) -> &str{ &self.key }
    pub fn is_old(&self) -> bool{ self.is_old }
}

fn to_key_items(kvs : &DataKVs) -> Vec<KeyItem>{
    kvs.items().iter().map(|a| KeyItem::new(a.id().to_string(), a.is_old())).collect()
}

pub fn get_member_desc(root : *const RootObject) -> Vec<MemberDesc>{
    let root = unsafe{ root.as_ref().unwrap() };
    let mut vec : Vec<MemberDesc> = Vec::with_capacity(root.default().len());
    for (k,val) in root.default(){
        let mem = k.to_string();
        let is_old = root.old().contains(k);
        match val{
            RootValue::Param(p, vt) =>{
                vec.push(MemberDesc::new(mem, vt.clone(), p.type_num(), is_old, None));
            },
            RootValue::Data(d) =>{
                let children = get_list_def_desc(d.default());
                let kvs = get_kvs(d);
                let descs = MemberDescs::with_keys(children, to_key_items(&kvs));
                vec.push(MemberDesc::new(mem, VarType::Normal, RustMemberType::Data, is_old, Some(descs)))
            },
            RootValue::List(l) =>{
                let children = get_list_def_desc(l.default());
                let descs = MemberDescs::new(children);
                vec.push(MemberDesc::new(mem, VarType::Normal, RustMemberType::List, is_old, Some(descs)))
            },
            RootValue::Mut(m) =>{
                let children = get_list_def_desc(m.default());
                let descs = MemberDescs::new(children);
                vec.push(MemberDesc::new(mem, VarType::Normal, RustMemberType::Mut, is_old, Some(descs)))
            },
        };
    }
    vec
}

pub fn get_list_def_desc(def : &ListDefObj) -> Vec<MemberDesc>{
    let mut vec : Vec<MemberDesc> = Vec::with_capacity(def.default().len());
    for (k,val) in def.default(){
        let mem = k.to_string();
        let is_old = def.old().contains(k);
        let (mt, vt, def) = match val{
            ListDefValue::Param(p, vt) =>{
                (p.type_num(), vt.clone(), None)
            },
            ListDefValue::InnerDataDef(d) => (RustMemberType::InnerData, VarType::Normal, Some(get_list_def_desc(&d))),
            ListDefValue::InnerListDef(d) => (RustMemberType::InnerList, VarType::Normal, Some(get_list_def_desc(&d))),
            ListDefValue::InnerMutDef(d) =>
                (RustMemberType::InnerMut,
                 if d.undefinable(){ VarType::Undefiable } else{ VarType::Normal },
                 Some(get_list_def_desc(d.list_def()))),
        };
        vec.push(MemberDesc::new(mem, vt, mt, is_old, def.map(|d| MemberDescs::new(d))));
    }
    vec
}

