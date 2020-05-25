
use crate::structs::root_object::{RustObject, ListDefObj};
use std::collections::{BTreeMap, HashSet, HashMap};
use linked_hash_map::LinkedHashMap;
use std::rc::{Rc, Weak};
use crate::indexmap::IndexMap;
use crate::structs::rust_value::RustValue;
use crate::structs::ref_value::RefValue;


///アイテムごとにIDをもち、Refで参照することが可能である。InnerListはConstListとConstDataを使える。
/// InnerListでConstDataを使う場合、Refする手段がないので、プログラムからIDでアクセスする使いみちぐらいしかなさそう。
#[derive(Debug, PartialEq)]
pub struct ConstData{
    pub default : ListDef,
    pub list : IndexMap<String, ListItem>,
    ///oldに設定されたIDはjsonから参照出来ない。変数名の末尾に"_Old"をつけないとプログラムからも使えない。
    pub old : HashSet<String>,
}

///IDを持たず、参照できない。ConstDataのInnerListで使うのが主な使いみちだけれど、IDはいらないけど単にデータを書いておきたい場合もあるだろう。
/// InnerListとしてConstDataとConstListを持てる。DefaultではDefaultだけを書き、ListItemのInnerListではDefaultをWeakで参照しつつ、ConstなListをその都度書くことになる。
#[derive(Debug, PartialEq)]
pub struct ConstList{
    pub default : ListDef,
    pub list : Vec<ListItem>,
}

///参照できない。MutListにコピーするのが使いみち。InnerListはMutListのみ持てる。InnerListにされることはない。
#[derive(Debug, PartialEq)]
pub struct InitialList{
    pub default : ListDef,
    pub list : Vec<ListItem>,
    ///MutListは初期値を持てないのでInitialListに初期値を書いておくことになるだろう。
    /// その場合、compatibleを設定しdefaultが同一であることを保証することで、そのままListItemをコピーすることが可能になる
    pub compatible : Vec<String>,
}

///追加、削除、順番の変更等ができるリスト。初期値を持てず最初は必ず空リストである。これはバージョン違いを読み出す時に問題を単純化するために必要。
///InnerListはMutListのみである。Constのものを内部に抱える必要はなかろうと思う。「生成後変更できないようにしたい」という需要はあるかもしれないが・・・このシステムにとってはあんまり本質的ではないと思う。
#[derive(Debug, PartialEq)]
pub struct MutList{
    pub default : ListDef,
    pub list : Vec<MutListItem>,
    ///追加される度にこのIDがふられ、これがインクリメントされることを徹底する必要がある。u64を使い切るには1万年ぐらいかかるだろう
    pub next_id : u64,
}

pub struct ListItem{
    ///ListItemの値は常にDefaultからの差分である
    pub values : HashMap<String, RustValue>,
    ///ListItemの値はRefも常にDefaultからの差分である
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
    ///InnerListのDefaultは、ListItemのInnerListから参照されるので、Rcにする。
    InnerDef(Rc<ListDefObj>),
    ///ListItem内のInnerListは、InnerDefのDefaultを参照する形になる。別にRcでもいいが見た目に分かりやすいのでWeakにしておく
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