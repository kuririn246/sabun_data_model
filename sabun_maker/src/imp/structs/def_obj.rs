use std::collections::{HashSet, HashMap};
use crate::imp::structs::ref_value::RefValue;
use crate::imp::structs::list_value::ListDefValue;


#[derive(Debug, PartialEq, Clone)]
pub struct ListDefObj{
    default : Box<HashMap<String, ListDefValue>>,
    ///RustValueを巨大にしすぎないためにBoxにしてサイズを削る
    refs: Box<RefDefObj>,
    ///oldに設定されたメンバは、defaultでの初期値を覗いてjsonで値を入れられず、プログラムからも_Oldを付けないとアクセスできない
    old : Box<HashSet<String>>,
}

impl ListDefObj{
    pub(crate) fn new(default : HashMap<String, ListDefValue>, refs : RefDefObj, old : HashSet<String>) -> ListDefObj{
        ListDefObj{ default : Box::new(default), refs : Box::new(refs), old : Box::new(old) }
    }
    pub(crate) fn default(&self) -> &HashMap<String, ListDefValue>{ self.default.as_ref() }
    pub(crate) fn refs(&self) -> &RefDefObj{ self.refs.as_ref() }
    pub(crate) fn old(&self) -> &HashSet<String>{ self.old.as_ref() }

    pub(crate) fn compatible(&self, other : &Self) -> bool{
        for (k,v) in self.default(){
            match other.default.get(k){
                Some(v2) =>{
                    if v.compatible(v2) == false{
                        return false;
                    }
                },
                None =>{ return false; }
            }
        }
        return self.refs.compatible(other.refs())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct RefDefObj {
    refs: Box<HashMap<String, RefValue>>,
    /// Enum とRefの二通りの定義の仕方があり、Enumの場合は Ref のうち一つだけ値があり、ほかは全部nullにしなきゃいけない。
    /// プログラムからはmatch でアクセス出来る。値があるRefをキャストしてゲットする。
    is_enum : bool,
    ///oldに設定されたメンバは、defaultでの初期値を覗いてjsonで値を入れられず、プログラムからも_Oldを付けないとアクセスできない
    old : Box<HashSet<String>>,
}

impl RefDefObj{
    pub(crate) fn new(refs : HashMap<String, RefValue>, is_enum : bool, old : HashSet<String>) -> RefDefObj{
        RefDefObj{ refs : Box::new(refs), is_enum, old : Box::new(old) }
    }
    pub(crate) fn refs(&self) -> &HashMap<String, RefValue>{ self.refs.as_ref() }
    pub(crate) fn old(&self) -> &HashSet<String>{ self.old.as_ref() }
    pub(crate) fn is_enum(&self) -> bool{ self.is_enum }

    pub(crate) fn compatible(&self, other : &Self) -> bool{
        for (k,v) in self.refs(){
            match other.refs.get(k){
                Some(v2) =>{
                    if v.compatible(v2) == false{
                        return false;
                    }
                },
                None =>{ return false; }
            }
        }
        return true;
    }
}


#[derive(Debug, PartialEq, Clone)]
pub struct InnerMutDefObj {
    list_def : Box<ListDefObj>,
    undefinable: bool,
    compatible : Box<HashSet<String>>,
}

impl InnerMutDefObj{
    pub(crate) fn new(list_def : ListDefObj, undefinable : bool, compatible : HashSet<String>) -> InnerMutDefObj{
        InnerMutDefObj{ list_def : Box::new(list_def), undefinable, compatible : Box::new(compatible) }
    }
    pub(crate) fn list_def(&self) -> &ListDefObj{ self.list_def.as_ref() }
    pub(crate) fn undefinable(&self) -> bool{ self.undefinable }
    pub(crate) fn compatible(&self) -> &HashSet<String>{ self.compatible.as_ref() }
}

