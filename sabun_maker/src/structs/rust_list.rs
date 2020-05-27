
use crate::structs::root_object::{ListDefObj};
use std::collections::{HashSet, HashMap};
use crate::indexmap::IndexMap;
use crate::structs::rust_value::RustValue;
use crate::structs::ref_value::RefValue;


///アイテムごとにIDをもち、Refで参照することが可能である
#[derive(Debug, PartialEq)]
pub struct ConstData{
    pub default : ListDefObj,
    pub list : IndexMap<String, ListItem>,
    ///oldに設定されたIDはjsonから参照出来ない。変数名の末尾に"_Old"をつけないとプログラムからも使えない。
    pub old : HashSet<String>,
}

///IDを持たず、参照できない。MutListの初期値を書くのが主な使い道か。IDは必要ないけど単にデータを書いておきたい場合もあるだろう。
#[derive(Debug, PartialEq)]
pub struct ConstList{
    pub default : ListDefObj,
    pub list : Vec<ListItem>,
    ///MutListは初期値を持てないのでConstListに初期値を書いておくことになるだろう。
    /// その場合、compatibleを設定しdefaultが同一であることを保証することで、そのままListItemをコピーすることが可能になる
    pub compatible : HashSet<String>,
}

///追加、削除、順番の変更等ができるリスト。初期値を持てず最初は必ず空リストである。これはバージョン違いを読み出す時に問題を単純化するために必要。
/// ConstListとMutListはstruct定義を見ると近い存在なので、まとめてもいいように思うかもしれないけれど、意味が全く別ものなので型を分けたほうが混乱が少ない。
/// 順序を変えなければidでソートされたSortedListになるのでPrimaryKeyを持ったTableとしても使えないこともないか
#[derive(Debug, PartialEq)]
pub struct MutList{
    pub default : ListDefObj,
    pub list : Vec<MutListItem>,
    ///追加される度にこのIDがふられ、これがインクリメントされることを徹底する必要がある。u64を使い切るには1万年ぐらいかかるだろう
    pub next_id : u64,
}

///Data or Listの内部に作るList。上がMutの場合はMutだし、ConstならConstだが、アクセス制限の違いだけで中身の違いはない。Constの場合idは無視できる。ListDefObjにはDefaultだけ書き、ListItemでは必要ならItemのみを書く。
///MutListをTableとして、InnerListをSortedListとして運用することで理論上はRelationも表現できる・・・でも実際はConstDataとRefでやるべきだと思う
#[derive(Debug, PartialEq)]
pub struct InnerList{
    pub list : Vec<MutListItem>,
    ///追加される度にこのIDがふられ、これがインクリメントされることを徹底する必要がある。u64を使い切るには1秒間に1億生成しても1万年ぐらいかかるはず
    /// Relationに使うつもりなら、next_idを振る必要はないだろう。その場合、u64::MAX_VALUEをnext_idに入れておくことにしようか
    /// next_idをインクリメントする時、u64::MAXを超えるときはpanicするようにシステムを組むと、バグが検出できてよろしかろう。
    pub next_id : u64,
}

#[derive(Debug, PartialEq)]
pub struct ListItem{
    ///ListItemの値は常にDefaultからの差分である
    pub values : HashMap<String, RustValue>,
    ///ListItemの値はRefも常にDefaultからの差分である
    pub refs : HashMap<String, RefValue>,
}

///たとえばキャラクターAとキャラクターBの間で出来事Cが起こったとする。
/// キャラクターAのIDをa, BのIDをbとする。
/// グローバルの出来事リストに出来事Cを記録し、next_idからidを取り、そのidをcとする。その出来事のオブジェクトにはaとbもvaluesに記録されている。
/// AのインナーリストのID bの項目にアクセスし、なければ作成し、insertする。
/// Aのbの下にある出来事ID保持用のinner listに出来事ID cを記憶しておく。ID保持用のinner listは、idだけで中身のないオブジェクトを集めたinner listになる。
/// 同様にキャラクターBのRelationリストaの出来事リストにも、出来事ID cを記録。
/// これにより、たとえば出来事Cを削除したい場合、Cにあるaとbを読み、AのbにあるID cのものを削除、 Bのaにあるcも削除、さらに出来事リストからCも削除すると、全部消える。
/// AとBとの間で何があったかの一覧がほしいなら、Aのbにアクセスし、出来事IDリストを取得、出来事リストからSortedListの2分探索を用いてid検索し、出来事を取得、という感じになる。
/// 出来事リストのIDはnext_id方式により、時系列に積み上がっていくため、何年何月に起きた出来事はID x から y という情報があれば、
/// その間の出来事を全部調べたり、一定期間が過ぎた出来事データのうち重要じゃないものは消す、といった処理もできる。
/// キャラクターBを削除したい場合、他のキャラクターのinner listのbの部分を全部消し、Bのインナーリストから取れる出来事IDを全部調べてHashSetに入れて、
/// 出来事リストを舐めながら全部消す、といった感じで消していくことが可能だ。
#[derive(Debug, PartialEq)]
pub struct MutListItem{
    ///アイテムごとにidが振られ、これによって削除や順番の変更を検出できる
    pub id : u64,
    ///ListItemの値は常にDefaultからの差分である
    pub values : IndexMap<String, RustValue>,
    ///ListItemの値はRefでも常にDefaultからの差分である
    pub refs : IndexMap<String, RefValue>,
}


