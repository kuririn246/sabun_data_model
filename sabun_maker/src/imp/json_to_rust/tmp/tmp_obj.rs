use crate::structs::rust_value::RustValue;
use crate::indexmap::IndexMap;
use crate::structs::ref_value::RefValue;

pub struct TmpObj{
    pub default : IndexMap<String, RustValue>,
    pub id : Option<String>,
    pub include : Vec<String>,
    pub refs: IndexMap<String, RefValue>,
}

impl TmpObj{
    pub fn new() -> TmpObj{
        TmpObj{ default : IndexMap::new(), id : None, include : vec![], refs : IndexMap::new() }
    }

    pub fn insert_default(&mut self, s : String, v : RustValue){
        self.default.insert(s, v);
    }
}