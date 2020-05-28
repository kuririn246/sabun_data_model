use crate::structs::rust_value::RustValue;
use crate::indexmap::IndexMap;
use crate::structs::ref_value::RefValue;
use std::collections::HashSet;
use crate::structs::rust_list::ListItem;

pub struct TmpObj{
    pub default : IndexMap<String, RustValue>,
    pub id : Option<IdValue>,
    pub include : Vec<String>,
    pub refs: TmpRefs,
    pub old : HashSet<String>,
}

pub struct TmpRefs{
    pub map : IndexMap<String, RefValue>,
    pub old : HashSet<String>,
}

pub enum IdValue{
    Str(String),
    Num(f64)
}

impl TmpObj{
    pub fn new() -> TmpObj{
        TmpObj{ default : IndexMap::new(), id : None, include : vec![], refs : TmpRefs{ map : IndexMap::new(), old : HashSet::new() }, old : HashSet::new() }
    }

    pub fn insert_default(&mut self, s : String, v : RustValue){
        self.default.insert(s, v);
    }

    pub fn to_list_item(self) -> ListItem{
        ListItem{ refs : self.refs, values : self.default }
    }
}