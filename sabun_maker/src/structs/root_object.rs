use crate::structs::rust_value::{RustValue, RustParam};
use std::collections::{HashSet, HashMap};
use crate::structs::ref_value::RefValue;

#[derive(Debug, PartialEq)]
pub struct RootObject{
    //listのobjectの場合、defaultはlist側にあるが、ここには初期値が入る。
    pub default : HashMap<String, RustValue>,
    //変更されたものを記録。差分変更時に、defaultと同じになったらここから削除する
    //listの変更はRustListが直接上書きされるので、sabunには入らない
    pub sabun : HashMap<String, RustParam>,

    ///oldに設定されたメンバは、_Oldを付けなければプログラムから使用できず、
    ///ConstDataである場合、jsonで Refできない
    pub old : HashSet<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ListDefObj{
    pub default : Box<HashMap<String, RustValue>>,
    ///RustValueを巨大にしすぎないためにBoxにしてサイズを削る
    pub refs: Box<RefDefObj>,
    ///oldに設定されたメンバは、defaultでの初期値を覗いてjsonで値を入れられず、プログラムからも_Oldを付けないとアクセスできない
    pub old : Box<HashSet<String>>,
}

impl ListDefObj{
    pub fn default(&self) -> &HashMap<String, RustValue>{ self.default.as_ref() }
    pub fn refs(&self) -> &RefDefObj{ self.refs.as_ref() }
    pub fn old(&self) -> &HashSet<String>{ self.old.as_ref() }
}

#[derive(Debug, PartialEq, Clone)]
pub struct RefDefObj {
    pub refs: Box<HashMap<String, RefValue>>,
    /// Enum とRefの二通りの定義の仕方があり、Enumの場合は Ref のうち一つだけ値があり、ほかは全部nullにしなきゃいけない。
    /// プログラムからはmatch でアクセス出来る。値があるRefをキャストしてゲットする。
    pub is_enum : bool,
    ///oldに設定されたメンバは、defaultでの初期値を覗いてjsonで値を入れられず、プログラムからも_Oldを付けないとアクセスできない
    pub old : Box<HashSet<String>>,
}

impl RefDefObj{
    pub fn refs(&self) -> &HashMap<String, RefValue>{ self.refs.as_ref() }
    pub fn old(&self) -> &HashSet<String>{ self.old.as_ref() }
    pub fn is_enum(&self) -> bool{ self.is_enum }
}


#[derive(Debug, PartialEq, Clone)]
pub struct InnerMutDefObj {
    pub list_def : Box<ListDefObj>,
    pub undefinable: bool,
    pub compatible : Box<HashSet<String>>,
}

impl InnerMutDefObj{
    pub fn list_def(&self) -> &ListDefObj{ self.list_def.as_ref() }
    pub fn undefinable(&self) -> bool{ self.undefinable }
    pub fn compatible(&self) -> &HashSet<String>{ self.compatible.as_ref() }
}

