use crate::structs::rust_value::{RustValue, RustParam};
use std::collections::{HashSet, HashMap};
use crate::indexmap::IndexMap;
use crate::structs::ref_value::RefValue;

#[derive(Debug, PartialEq)]
pub struct RootObject{
    //別ファイルにあったことを記録しておくためのもの。どう使うかは後で考える。
    pub include : Vec<String>,
    //listのobjectの場合、defaultはlist側にあるが、ここには初期値が入る。
    pub default : IndexMap<String, RustValue>,
    //変更されたものを記録。差分変更時に、defaultと同じになったらここから削除する
    //listの変更はRustListが直接上書きされるので、sabunには入らない
    pub sabun : HashMap<String, RustParam>,

    ///oldに設定されたメンバは、_Oldを付けなければプログラムから使用できず、
    ///ConstDataである場合、jsonで Refできない
    pub old : HashSet<String>,
}

#[derive(Debug, PartialEq)]
pub struct ListDefObj{
    pub default : IndexMap<String, RustValue>,
    pub refs: IndexMap<String, RefValue>,
    ///oldに設定されたメンバは、defaultでの初期値を覗いてjsonで値を入れられず、プログラムからも_Oldを付けないとアクセスできない
    pub old : HashSet<String>,
}

impl ListDefObj{
    pub fn new() -> ListDefObj{
        ListDefObj{ default : IndexMap::new(), refs : IndexMap::new(), old : HashSet::new() }
    }
}

//
//
// impl RootObject {
//     pub fn new() -> RootObject {
//         RustObject {
//             include: vec![],
//             default: IndexMap::new(),
//             sabun: HashMap::new(),
//             obsolete: false
//         }
//     }
// }
//
//     pub fn insert_default(&mut self, key : String, value : RustValue) -> Option<RustValue>{
//         return self.default.insert(key, value);
//     }
//
//     pub fn get_list(&self, name : &str) -> Option<&RustList>{
//         match self.default.get(name){
//             Some(RustValue::List(l)) =>{
//                 Some(l)
//             },
//             _ => None,
//         }
//     }
// }