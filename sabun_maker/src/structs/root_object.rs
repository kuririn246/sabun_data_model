use crate::structs::rust_value::{RustValue, RustParam};
use std::collections::{HashSet, HashMap};
use crate::structs::ref_value::RefValue;
use crate::indexmap::str_vec_map::StrVecMap;

#[derive(Debug, PartialEq)]
pub struct RootObject{
    //別ファイルにあったことを記録しておくためのもの。どう使うかは後で考える。
    pub include : Vec<String>,
    //listのobjectの場合、defaultはlist側にあるが、ここには初期値が入る。
    pub default : StrVecMap<RustValue>,
    //変更されたものを記録。差分変更時に、defaultと同じになったらここから削除する
    //listの変更はRustListが直接上書きされるので、sabunには入らない
    pub sabun : HashMap<String, RustParam>,

    ///oldに設定されたメンバは、_Oldを付けなければプログラムから使用できず、
    ///ConstDataである場合、jsonで Refできない
    pub old : HashSet<String>,
}

#[derive(Debug, PartialEq)]
pub struct ListDefObj{
    pub default : StrVecMap<RustValue>,
    pub refs: RefDefObj,
    ///oldに設定されたメンバは、defaultでの初期値を覗いてjsonで値を入れられず、プログラムからも_Oldを付けないとアクセスできない
    pub old : HashSet<String>,
}



#[derive(Debug, PartialEq)]
pub struct RefDefObj {
    pub refs: StrVecMap<RefValue>,
    /// Enum とRefの二通りの定義の仕方があり、Enumの場合は Ref のうち一つだけ値があり、ほかは全部nullにしなきゃいけない。
    /// プログラムからはmatch でアクセス出来る。値があるRefをキャストしてゲットする。
    pub is_enum : bool,
    ///oldに設定されたメンバは、defaultでの初期値を覗いてjsonで値を入れられず、プログラムからも_Oldを付けないとアクセスできない
    pub old : HashSet<String>,
}

