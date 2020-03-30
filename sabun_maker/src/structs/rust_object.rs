use crate::structs::rust_value::RustValue;
use std::collections::BTreeMap;
use indexmap::IndexMap;
use crate::structs::ref_value::RefValue;

#[derive(Debug)]
pub struct RustObject{
    //別ファイルにあったことを記録しておくためのもの。どう使うかは後で考える。
    pub include : Vec<String>,
    //listのobjectの場合、defaultはlist側にあるが、ここには初期値が入る。
    pub default : IndexMap<String, RustValue>,
    //変更されたものを記録。差分変更時に、defaultと同じになったらここから削除する
    pub sabun : IndexMap<String, RustValue>,
    //listの場合idがなければならず、list内で一意である必要もある。
    //listのオブジェクトでない場合はNone
    pub id : Option<String>,
    pub refs: Option<IndexMap<String, RefValue>>,
    pub renamed: BTreeMap<String, String>,
    pub obsolete : bool,
}


impl RustObject{
    pub fn new() -> RustObject{
        RustObject{
            include : vec![],
            default : Some(IndexMap::new()), sabun : IndexMap::new(),
            id : None, refs: None,
            renamed: BTreeMap::new(), obsolete : false }
    }

    pub fn insert_default(&mut self, key : String, value : RustValue) -> Option<RustValue>{
        if self.default.is_none(){
            self.default = Some(IndexMap::new());
        }
        let def = self.default.as_mut().unwrap();
        return def.insert(key, value);
    }
}