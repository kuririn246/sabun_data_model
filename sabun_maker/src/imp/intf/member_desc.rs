use crate::imp::structs::value_type::VarType;
use crate::imp::structs::rust_value::RustMemberType;
use crate::imp::structs::list_value::ListDefValue;
use crate::imp::structs::list_def_obj::ListDefObj;
use crate::imp::structs::root_obj::RootObject;
use crate::imp::structs::root_value::RootValue;
use crate::imp::intf::data::{get_kvs, DataKVs, ConstDataPtr};
use crate::imp::intf::ref_desc::{RefDescs, get_ref_def_desc};

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
    keys : Vec<KeyItem>,
    refs : RefDescs,
}
impl MemberDescs{
    pub(crate) fn new(items : Vec<MemberDesc>, refs : RefDescs) -> MemberDescs{ MemberDescs{ items, keys : vec![], refs }}
    pub(crate) fn with_keys(items : Vec<MemberDesc>, refs : RefDescs, keys : Vec<KeyItem>) -> MemberDescs{
        MemberDescs{ items, refs, keys }
    }
    pub fn items(&self) -> &[MemberDesc]{ &self.items }
    pub fn keys(&self) -> &[KeyItem]{ &self.keys }
    pub fn refs(&self) -> &RefDescs{ &self.refs }
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
                let refs = get_ref_def_desc(d.default().refs());
                let kvs = get_kvs(ConstDataPtr::new(d));
                let descs = MemberDescs::with_keys(children, refs, to_key_items(&kvs));
                vec.push(MemberDesc::new(mem, VarType::Normal, RustMemberType::Data, is_old, Some(descs)))
            },
            RootValue::List(l) =>{
                let children = get_list_def_desc(l.default());
                let refs = get_ref_def_desc(l.default().refs());
                let descs = MemberDescs::new(children, refs);
                vec.push(MemberDesc::new(mem, VarType::Normal, RustMemberType::List, is_old, Some(descs)))
            },
            RootValue::Mut(m) =>{
                let children = get_list_def_desc(m.default());
                let refs = get_ref_def_desc(m.default().refs());
                let descs = MemberDescs::new(children, refs);
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
        match val{
            ListDefValue::Param(p, vt) =>{
                vec.push(MemberDesc::new(mem, vt.clone(), p.type_num(),
                                         is_old, None));
            },
            ListDefValue::InnerDataDef(d) =>{
                let ld = get_list_def_desc(d);
                let rd = get_ref_def_desc(d.refs());
                vec.push(MemberDesc::new(
                    mem, VarType::Normal, RustMemberType::InnerData,
                    is_old,Some(MemberDescs::new(ld, rd))));
            },
            ListDefValue::InnerListDef(d) =>{
                let ld = get_list_def_desc(d);
                let rd = get_ref_def_desc(d.refs());
                vec.push(MemberDesc::new(
                    mem, VarType::Normal, RustMemberType::InnerList,
                    is_old,Some(MemberDescs::new(ld, rd))));
            },
            ListDefValue::InnerMutDef(d) =>{
                let ld = get_list_def_desc(d.list_def());
                let rd = get_ref_def_desc(d.list_def().refs());
                let vt = if d.undefinable(){ VarType::Undefiable } else{ VarType::Normal };
                vec.push(MemberDesc::new(
                    mem, vt, RustMemberType::InnerMut,
                         is_old,Some(MemberDescs::new(ld, rd))));
            }
        }
    }
    vec
}

