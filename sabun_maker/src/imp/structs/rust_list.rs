
use crate::{HashM, HashS, HashMt};
use crate::imp::structs::ref_value::RefSabValue;
use crate::imp::structs::list_value::{ListSabValue, ListDefValue};
use crate::imp::structs::rust_param::RustParam;
use crate::imp::structs::util::set_sabun::{SetSabunError, verify_set_sabun};
use crate::imp::structs::list_def_obj::ListDefObj;
use crate::imp::structs::linked_m::LinkedMap;


///アイテムごとにIDをもち、Refで参照することが可能である
#[derive(Debug, PartialEq, Clone)]
pub struct ConstData{
    default : Box<ListDefObj>,
    list : Box<HashM<String, ListItem>>,
    ///oldに設定されたIDはjsonから参照出来ない。変数名の末尾に"_Old"をつけないとプログラムからも使えない。
    old : Box<HashS<String>>,
}

impl ConstData{
    pub(crate) fn new(default : ListDefObj, list : HashM<String, ListItem>, old : HashS<String>) -> ConstData{
        ConstData{ default : Box::new(default), list : Box::new(list), old : Box::new(old) }
    }
    pub(crate) fn default(&self) -> &ListDefObj{ self.default.as_ref() }
    pub(crate) fn list(&self) -> &HashM<String, ListItem>{ self.list.as_ref() }
    pub(crate) fn old(&self) -> &HashS<String>{ self.old.as_ref() }
}

///IDを持たず、参照できない。MutListの初期値を書くのが主な使い道か。IDは必要ないけど単にデータを書いておきたい場合もあるだろう。
#[derive(Debug, PartialEq, Clone)]
pub struct ConstList{
    default : Box<ListDefObj>,
    list : Box<Vec<ListItem>>,
}

impl ConstList{
    pub(crate) fn new(default : ListDefObj, list : Vec<ListItem>) -> ConstList{ ConstList{ default : Box::new(default), list : Box::new(list) } }
    pub(crate) fn default(&self) -> &ListDefObj{ self.default.as_ref() }
    pub(crate) fn list(&self) -> &Vec<ListItem>{ self.list.as_ref() }
}

///追加、削除、順番の変更等ができるリスト。初期値を持てず最初は必ず空リストである。これはバージョン違いを読み出す時に問題を単純化するために必要。
/// ConstListとMutListはstruct定義を見ると近い存在なので、まとめてもいいように思うかもしれないけれど、意味が全く別ものなので型を分けたほうが混乱が少ない。
/// 順序を変えなければidでソートされたSortedListになるのでPrimaryKeyを持ったTableとしても使えないこともないか
#[derive(Debug, PartialEq, Clone)]
pub struct MutList{
    default : Box<ListDefObj>,
    list : Box<LinkedMap<MutListItem>>,
    compatible : Box<HashS<String>>,
}

impl MutList{
    pub(crate) fn new(default : ListDefObj, list : LinkedMap<MutListItem>, compatible : HashS<String>) -> MutList{
        MutList{ default : Box::new(default), list : Box::new(list), compatible : Box::new(compatible) }
    }
    pub(crate) fn default(&self) -> &ListDefObj{ self.default.as_ref() }
    pub(crate) fn list(&self) -> &LinkedMap<MutListItem>{ self.list.as_ref() }
    pub(crate) fn distribute_mut(&mut self) -> (&mut ListDefObj, &mut LinkedMap<MutListItem>, &mut HashS<String>){ (self.default.as_mut(), self.list.as_mut(), self.compatible.as_mut()) }
    pub(crate) fn next_id(&self) -> u64{ self.list.as_ref().next_id() }
    //pub(crate) fn list_mut(&mut self) -> &mut LinkedHashM<u64, MutListItem>{ self.list.as_mut() }
    //pub(crate) fn increment_next_id(&mut self){ self.prop.next_id += 1 }
    pub(crate) fn append_new_item(&mut self) -> u64{
        self.list.insert(MutListItem::construct(HashMt::new(), HashMt::new()))
    }

    pub(crate) fn compatible(&self) -> &HashS<String>{ self.compatible.as_ref() }
    pub(crate) fn deconstruct(self) -> (ListDefObj, LinkedMap<MutListItem>, HashS<String>){
        (*self.default, *self.list, *self.compatible)
    }
}

///Data or Listの内部に作るList。ListDefObjの内部にはDefaultだけ書き、ListItemの内部にはItemのみを書く。
#[derive(Debug, PartialEq, Clone)]
pub struct InnerList{
    list : Vec<ListItem>,
}

impl InnerList{
    pub(crate) fn new(list : Vec<ListItem>) -> InnerList{ InnerList { list }}
    pub(crate) fn list(&self) -> &Vec<ListItem>{ &self.list }
}

#[derive(Debug, PartialEq, Clone)]
pub struct InnerMutList{
    list : Box<LinkedMap<MutListItem>>,
}

impl InnerMutList{
    pub(crate) fn new(list : LinkedMap<MutListItem>,) -> InnerMutList{ InnerMutList{ list : Box::new(list) } }
    pub(crate) fn deconstruct(self) -> LinkedMap<MutListItem>{ *self.list }
    pub(crate) fn list(&self) -> &LinkedMap<MutListItem>{ self.list.as_ref() }
    //pub(crate) fn next_id(&self) -> u64{ self.next_id }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ListItem{
    ///ListItemの値は常にDefaultからの差分である
    values : Box<HashM<String, ListSabValue>>,
    ///ListItemの値はRefも常にDefaultからの差分である
    refs : Box<HashM<String, RefSabValue>>,
}

impl ListItem{
    pub(crate) fn new(values : HashM<String, ListSabValue>, refs : HashM<String, RefSabValue>) -> ListItem{
        ListItem{ values : Box::new(values), refs : Box::new(refs) }
    }
    pub(crate) fn values(&self) -> &HashM<String, ListSabValue>{ self.values.as_ref() }
    pub(crate) fn refs(&self) -> &HashM<String, RefSabValue>{ self.refs.as_ref() }
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
/// こういったユースケース（あるのか？）のためにLinkedHashM(u64,MutListItem)を使うとRelationを効率的に処理できるだろう。
/// あるいはBTreeMap(index_value, u64)でindex_valueでソートされたMapを作り、「index_valueがAからBの間にあるアイテム」といった条件で検索が可能になる。
/// そういったシステムを、出来事リストを読み出して外部にRelationを構築したり、パラメータをindex-keyとしてBTreeを構築したりすることで
/// （パラメータは上書きされうるので、その場合(item_id, BTreeのid)のRelationも使って、上書き時にBTreeをアップデートできるようにしておく必要もあり大変だが)
/// Relationとパラメータ範囲での検索が効率的にできるシステムが作れる。ただそれは外部に作ればいいので、このシステム自体の守備範囲ではない
/// それが出来る土台として、idとLinkedHashMで出来たMutListがある
#[derive(Debug, PartialEq, Clone)]
pub struct MutListItem{
    /////アイテムごとにidが振られ、これによって削除や順番の変更を検出できる
    //id : u64,
    ///ListItemの値は常にDefaultからの差分である
    values : Box<HashM<String, ListSabValue>>,
    ///ListItemの値はRefでも常にDefaultからの差分である
    refs : Box<HashM<String, RefSabValue>>,
}

impl MutListItem{
    pub(crate) fn construct(values : HashM<String, ListSabValue>, refs : HashM<String, RefSabValue>) -> MutListItem{
        MutListItem{ values : Box::new(values), refs : Box::new(refs) }
    }
    pub(crate) fn new() -> MutListItem{
        MutListItem{ values : Box::new(HashMt::new()), refs : Box::new(HashMt::new()) }
    }
    pub(crate) fn deconstruct(self) -> (HashM<String, ListSabValue>, HashM<String, RefSabValue>){ (*self.values, *self.refs) }
    //pub(crate) fn id(&self) -> u64{ self.id }
    pub(crate) fn values(&self) -> &HashM<String, ListSabValue>{ self.values.as_ref() }
    pub(crate) fn refs(&self) -> &HashM<String, RefSabValue>{ self.refs.as_ref() }
    pub(crate) fn set_sabun(&mut self, def :&ListDefObj, name : String, param : RustParam) -> Result<Option<RustParam>, SetSabunError> {
        let (p, vt) =
            if let Some(ListDefValue::Param(p, vt)) = def.default().get(&name) {
                (p, vt)
            } else {
                return Err(SetSabunError::ParamNotFound);
            };
        verify_set_sabun(p, vt, &param)?;
        let op = self.values.insert(name, ListSabValue::Param(param));
        Ok(op.map(|v| match v{
            ListSabValue::Param(p) => p,
            _ => unreachable!(),
        }))
    }

}