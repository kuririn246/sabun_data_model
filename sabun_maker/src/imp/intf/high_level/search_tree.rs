use crate::imp::structs::root_obj::RootObject;
use crate::imp::structs::qv::{Qv, QvType};

pub enum SearchFromCol{
    ///ListとMutListで、Listの場合はvecのindex, MutListの場合はLinkedHashMapのIDで取得
    FromNumId(u64),
    FromStrID(String),
    FromNumIDRef(u64, String),
    FromStrIDRef(String, String),
}

///メンバ名からcolを取得、from_colからアイテムを取得、menber_name[1]からさらに取得、from_col[1],member_name[2]...
/// とvecが尽きるまで順々にサーチしていき、全部使い切ったら
pub struct SearchArgs{
    pub member_name : Vec<String>,
    pub from_col : Vec<SearchFromCol>,
    pub action : Action,
}

pub enum Action{
    GetLen, GetIndexes, GetIDs, GetMemberDesc, GetRefDesc, GetUnsafePtr,
    GetBool(String), GetNum(String), GetStr(String), GetArrayNum(String), GetArrayStr(String), GetArrayNum2(String),
    SetBool(String, Qv<bool>), SetNum(String, Qv<f64>), SetStr(String, Qv<String>),
    SetNumArray(String, Qv<Vec<f64>>), SetStrArray(String, Qv<Vec<String>>), SetNumArray2(String, Qv<Vec<Vec<f64>>>),
}

pub enum ActionResult{
    Len(u64), Indexes(Option<Vec<u64>>), IDs(Vec<String>), MemberDesc(Vec<MemberDesc>), RefDesc(Vec<RefDesc>),
    Bool(Qv<bool>), Num(Qv<f64>), Str(Qv<String>), ArrayNum(Qv<Vec<f64>>), ArrayStr(Qv<Vec<String>>), ArrayNum2(Qv<Vec<Vec<f64>>>),
}

pub struct ActionError{
    pub index : ActionErrorIndex,
    pub et : AET,
}

impl ActionError{
    pub fn member(index : usize, et : AET) -> ActionError{ ActionError{ index : ActionErrorIndex::Member(index), et } }
    pub fn from_col(index : usize, et : AET) -> ActionError{ ActionError{ index : ActionErrorIndex::Member(index), et } }
}

pub enum ActionErrorIndex{
    Member(usize), FromCol(usize), Action
}

///Action Error Type
pub enum AET{
    IndexOutOfRange
}

pub struct MemberDesc{
    pub member : String,
    pub qv_type : QvType,
    pub member_type : MemberType,
}

pub struct RefDesc{
    pub member : String,
    pub qv_type : QvType,
}

pub enum MemberType{
    Bool, Num, Str, NumArray, StrArray, NumArray2,
    Data, List, MutList, InnerData, InnerList, InnerMutList,
}

///低速だが、速度を求めるなら if(a == null){ a = search_from_root(hoge); a } else { a }で読み出すような形になるだろう
///それでも起動直後の速度を求めるなら、このインターフェースは不十分か？
/// しかしまあ、E*N回のハッシュテーブルアクセスが問題になるなら、それよりもハッシュテーブルのコンストラクトにかかる時間の方が問題になるはずだ
/// 仮にハッシュテーブルアクセスが頻発するのが問題になるユースケースなら上の方式を使えば速くなるだろう
pub fn search_from_root(root : &RootObject, args : SearchArgs) -> Result<ActionResult, ActionError> {
    if args.member_name.len() == 0 { return Err(ActionError::member(0, AET::IndexOutOfRange)); }
    let name = &args.member_name[i];
    root.

        ActionResult
    ::Len(1)
}