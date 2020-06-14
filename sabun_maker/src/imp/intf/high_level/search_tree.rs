use crate::imp::structs::root_obj::RootObject;
use crate::imp::structs::qv::Qv;

pub enum SearchFromCol{
    ///ListとMutListで、Listの場合はvecのindex, MutListの場合はLinkedHashMapのIDで取得
    FromNumId(usize),
    FromStrID(String),
    FromNumIDRef(usize, String),
    FromStrIDRef(String, String),
}

///メンバ名からcolを取得、from_colからアイテムを取得、menber_name[1]からさらに取得、from_col[1],member_name[2]...
/// とvecが尽きるまで順々にサーチしていき、全部使い切ったら
pub struct SearchArgs{
    pub member_name : Vec<String>,
    pub from_col : Vec<SearchFromCol>,
    pub get_value : GetValueArg,
}

pub enum GetValueArg{
    Len, Indexes, IDs,
    Bool(String), Num(String), Str(String), ArrayNum(String), ArrayStr(String), ArrayNum2(String)
}

pub enum GetValueResult{
    Len(usize), Indexes(Option<Vec<usize>>), IDs(Vec<String>),
    Bool(Qv<bool>), Num(Qv<f64>), Str(Qv<String>), ArrayNum(Qv<Vec<f64>>), ArrayStr(Qv<Vec<String>>), ArrayNum2(Qv<Vec<Vec<f64>>>),
}

///低速だが、速度を求めるなら if(a == null){ a = search_from_root(hoge); a } else { a }で読み出すような形になるだろう
///それでも起動直後の速度を求めるなら、このインターフェースは不十分か？
/// しかしまあ、E*N回のハッシュテーブルアクセスが問題になるなら、それよりもハッシュテーブルのコンストラクトにかかる時間の方が問題になるはずだ
/// 仮にハッシュテーブルアクセスが頻発するのが問題になるユースケースなら上の方式を使えば速くなるだろう
pub fn search_tree_from_root(root : &RootObject, args : SearchArgs) -> GetValueResult{
    todo!()
}