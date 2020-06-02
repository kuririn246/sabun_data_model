
use crate::structs::root_object::{ListDefObj};
use std::collections::{HashSet, HashMap};
use crate::structs::rust_value::RustValue;
use crate::structs::ref_value::RefValue;
use linked_hash_map::LinkedHashMap;


///アイテムごとにIDをもち、Refで参照することが可能である
#[derive(Debug, PartialEq, Clone)]
pub struct ConstData{
    pub default : ListDefObj,
    pub list : Box<HashMap<String, ListItem>>,
    ///oldに設定されたIDはjsonから参照出来ない。変数名の末尾に"_Old"をつけないとプログラムからも使えない。
    pub old : Box<HashSet<String>>,
}

///IDを持たず、参照できない。MutListの初期値を書くのが主な使い道か。IDは必要ないけど単にデータを書いておきたい場合もあるだろう。
#[derive(Debug, PartialEq, Clone)]
pub struct ConstList{
    pub default : Box<ListDefObj>,
    pub list : Vec<ListItem>,
}

///追加、削除、順番の変更等ができるリスト。初期値を持てず最初は必ず空リストである。これはバージョン違いを読み出す時に問題を単純化するために必要。
/// ConstListとMutListはstruct定義を見ると近い存在なので、まとめてもいいように思うかもしれないけれど、意味が全く別ものなので型を分けたほうが混乱が少ない。
/// 順序を変えなければidでソートされたSortedListになるのでPrimaryKeyを持ったTableとしても使えないこともないか
#[derive(Debug, PartialEq, Clone)]
pub struct MutList{
    pub default : Box<ListDefObj>,
    pub list : Box<LinkedHashMap<u64, MutListItem>>,
    ///追加される度にこのIDがふられ、これがインクリメントされることを徹底する必要がある。u64を使い切るには1万年ぐらいかかるだろう
    pub next_id : u64,

    ///MutListは初期値を持てないのでConstListに初期値を書いておくことになるだろう。
    /// その場合、compatibleを設定しdefaultが同一であることを保証することで、そのままListItemをコピーすることが可能になる
    pub compatible : Box<HashSet<String>>,
}

///Data or Listの内部に作るList。ListDefObjの内部にはDefaultだけ書き、ListItemの内部にはItemのみを書く。
#[derive(Debug, PartialEq, Clone)]
pub struct InnerList{
    pub list : Vec<ListItem>,
    //pub compatible : HashSet<String>,
}


///アイテムごとにIDをもち、Refで参照することが可能である
#[derive(Debug, PartialEq, Clone)]
pub struct InnerData{
    pub list : Box<HashMap<String, ListItem>>,
    ///oldに設定されたIDはjsonから参照出来ない。変数名の末尾に"_Old"をつけないとプログラムからも使えない。
    pub old : Box<HashSet<String>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct InnerMutList{
    pub list : Box<LinkedHashMap<u64, MutListItem>>,
    ///追加される度にこのIDがふられ、これがインクリメントされることを徹底する必要がある。u64を使い切るには1万年ぐらいかかるだろう
    pub next_id : u64,
}


#[derive(Debug, PartialEq, Clone)]
pub struct ListItem{
    ///ListItemの値は常にDefaultからの差分である
    pub values : Box<HashMap<String, RustValue>>,
    ///ListItemの値はRefも常にDefaultからの差分である
    pub refs : Box<HashMap<String, RefValue>>,
}

///たとえばキャラクターAとキャラクターBの間で出来事Cが起こったとする。
/// キャラクターAのIDをa, BのIDをbとする。
/// グローバルの出来事リストに出来事Cを記録し、next_idからidを振り、そのidをcとする。その出来事のオブジェクトにはaとbもvaluesに記録されている。
/// AのRelationリストのID bの項目にアクセスし、なければ作成し、insertする。
/// AのRelationリストbにある出来事ID保持用のlistに出来事ID cを記憶しておく。ID保持用のlistは、idだけで中身のないオブジェクトを集めたlistだ。
/// 同様にキャラクターBのRelationリストaの出来事リストにも、出来事ID cを記録。
/// これにより、たとえば出来事Cを削除したい場合、Cにあるaとbを読み、AのbにあるID cのものを削除、 Bのaにあるcも削除、さらに出来事リストからCも削除すると、全部消える。
/// AとBとの間で何があったかの一覧がほしいなら、Aのbにアクセスし、出来事IDリストを取得、出来事リストからid検索し、出来事を取得、という感じになる。
/// 出来事リストのIDはnext_id方式により、時系列に積み上がっていくため、何年何月に起きた出来事はID x から y という情報があれば、
/// その間の出来事を全部調べたり、一定期間が過ぎた出来事データのうち重要じゃないものは消す、といった処理もできる。
/// キャラクターBを削除したい場合、他のキャラクターのRelationリストのbの部分を全部消し、BのRelationリストから取れる出来事IDを全部調べて
/// 出来事リストから全部消す、といった感じで消していくことが可能だ。
///
/// こういったユースケース（あるのか？）のためにLinkedHashMap(u64,MutListItem)を使うとRelationを効率的に処理できるだろう。
/// あるいはBTreeMap(index_value, u64)でindex_valueでソートされたMapを作り、「index_valueがAからBの間にあるアイテム」といった条件で検索が可能になる。
/// そういったシステムを、出来事リストを読み出して外部にRelationを構築したり、パラメータをindex-keyとしてBTreeを構築したりすることで
/// （パラメータは上書きされうるので、その場合(item_id, BTreeのid)のRelationも使って、上書き時にBTreeをアップデートできるようにしておく必要もあり大変だが)
/// Relationとパラメータ範囲での検索が効率的にできるシステムが作れる。ただそれは外部に作ればいいので、このシステム自体の守備範囲ではない
/// それが出来る土台として、idとLinkedHashMapで出来たMutListがある
#[derive(Debug, PartialEq, Clone)]
pub struct MutListItem{
    ///アイテムごとにidが振られ、これによって削除や順番の変更を検出できる
    pub id : u64,
    ///ListItemの値は常にDefaultからの差分である
    pub values : Box<HashMap<String, RustValue>>,
    ///ListItemの値はRefでも常にDefaultからの差分である
    pub refs : Box<HashMap<String, RefValue>>,
}
