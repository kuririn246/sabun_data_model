use crate::structs::root_object::{RootObject};
use crate::error::Result;
use crate::structs::rust_value::RustValue;
use crate::imp::json_to_rust::validation::validate_data::validate_data;
use crate::imp::json_to_rust::names::Names;
use crate::imp::json_to_rust::validation::validate_list::validate_list;
use crate::imp::json_to_rust::validation::validate_mut_list::validate_mut_list;

/// json読み出し時のチェックがあり、adjust時のチェックもあり、modifyインターフェース上のチェックもある。
/// ここではどこまでチェックするのか、というのが問題なのだけど、基本的には全部チェックする、他のチェックに依存しない
/// というところを目指すことになるだろう。ダブルチェック、トリプルチェックや。
/// 静的型により可能な値はかなり限られるので、その中ですべてが妥当な値であるかどうかを調べることも出来ないことはないはず。
/// 基本的にrefが正しくoldじゃないものを指せているか調べる
/// can_ref_oldがあるとoldでも気にしなくなる
pub fn validate_root(root : &RootObject, can_ref_old : bool) -> Result<()>{
    for (name, val) in root.default(){
        let names = &Names::new(name);
        //RootはOldでも値を入れざるを得ないので入れて良い
        //なのでここではOldは無視
        match val {
            RustValue::Param(p, _) => {
                if let Some(sab) = root.sabun().get(name){
                    if p.acceptable(sab) == false{
                        Err(format!("Root's member {}'s sabun is invalid", name))?
                    }
                }
            },
            RustValue::Data(data) =>{
                validate_data(data.default(), data.list(), root, can_ref_old, names)?
            },
            RustValue::List(list) =>{
                validate_list(list.default(), list.list(), root, can_ref_old, names)?
            },
            RustValue::Mut(m) =>{
                validate_mut_list(m.default(), m.list(), m.compatible(), root, can_ref_old, names)?
            },
            RustValue::InnerData(_) => { Err(format!("{} : InnerData must not be defined in the root object", name))? },
            RustValue::InnerList(_) => { Err(format!("{} : InnerList must not be defined in the root object", name))? },
            RustValue::InnerMut(_) => { Err(format!("{} : InnerMut must not be defined in the root object", name))? },
            RustValue::InnerDataDef(_) => { Err(format!("{} : InnerDataDef must not be defined in the root object", name))? },
            RustValue::InnerListDef(_) => { Err(format!("{} : InnerListDef must not be defined in the root object", name))? },
            RustValue::InnerMutDef(_) => { Err(format!("{} : InnerMutDef must not be defined in the root object", name))? },
        }
    }

    return Ok(());
}