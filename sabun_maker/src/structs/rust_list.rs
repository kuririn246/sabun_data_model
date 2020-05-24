
use crate::structs::root_object::{RustObject, ListDefObj};
use std::collections::{BTreeMap, HashSet, HashMap};
use linked_hash_map::LinkedHashMap;
use std::rc::{Rc, Weak};
use crate::indexmap::IndexMap;
use crate::structs::rust_value::RustValue;
use crate::structs::ref_value::RefValue;


///アイテムごとにIDをもち、Refで参照することが可能である
#[derive(Debug, PartialEq)]
pub struct ConstData{
    pub default : ListDef,
    pub list : IndexMap<String, ListItem>,
    ///oldに設定されたIDはjsonから参照出来ない。変数名の末尾に"_Old"をつけないとプログラムからも使えない。
    pub old : HashSet<String>,
}

///参照できない。MutListにコピーするのが主な使いみち
#[derive(Debug, PartialEq)]
pub struct ConstList{
    pub default : ListDef,
    pub list : Vec<ListItem>,
    ///MutListは初期値を持てないのでConstListに初期値を書いておくことになるだろう。
    /// その場合、compatibleを設定しdefaultが同一であることを保証することで、そのままListItemをコピーすることが可能になる
    pub compatible : Vec<String>,
}

///追加、削除、順番の変更等ができるリスト。初期値を持てず最初は必ず空である。これはバージョン違いを読み出す時に問題を単純化するために必要。
#[derive(Debug, PartialEq)]
pub struct MutList{
    pub default : ListDef,
    pub list : Vec<MutListItem>,
    pub next_id : u64,
}

pub struct ListItem{
    ///ListItemの値は常にDefaultからの差分である
    pub values : HashMap<String, RustValue>,
    ///ListItemの値はRefでも常にDefaultからの差分である
    pub refs : HashMap<String, RefValue>,
}

pub struct MutListItem{
    ///アイテムごとにidが振られ、これによって削除や順番の変更を検出できる
    pub id : u64,
    ///ListItemの値は常にDefaultからの差分である
    pub values : IndexMap<String, RustValue>,
    ///ListItemの値はRefでも常にDefaultからの差分である
    pub refs : IndexMap<String, RefValue>,
}


#[derive(Debug, PartialEq)]
pub enum ListDef{
    Def(ListDefObj),
    InnerDef(Rc<ListDefObj>),
    InnerItem(Weak<ListDefObj>),
}
//
// impl RustList{
//     pub fn is_auto_id(&self) -> bool{
//         match self.list_type{
//             ListType::AutoID(_) => true,
//             _ => false,
//         }
//     }
//
//     pub fn is_null_auto_id(&self) -> bool{
//         match self.list_type{
//             ListType::AutoID(val) =>{
//                 val.is_none()
//             },
//             _ =>{ false }
//         }
//     }
//
//     pub fn set_auto_id(&mut self, id : u64) -> Result<(), ()>{
//         match self.list_type{
//             ListType::AutoID(_) =>{
//                 self.list_type = ListType::AutoID(Some(id));
//                 Ok(())
//             },
//             _=>{ Err(()) }
//         }
//     }
//
//     pub fn new() -> RustList{
//         RustList{
//             list_type : ListType::Normal,
//             default : ListDef::Def(RustObject::new()),
//             list : LinkedHashMap::new(),
//             redef : BTreeMap::new(),
//         }
//     }
// }