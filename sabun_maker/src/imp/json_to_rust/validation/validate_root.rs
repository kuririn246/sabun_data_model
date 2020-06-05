use crate::structs::root_object::{RootObject};
use crate::error::Result;
use crate::structs::rust_value::RustValue;
use crate::imp::json_to_rust::validation::validate_data::validate_data;
use crate::imp::json_to_rust::names::Names;
use crate::imp::json_to_rust::validation::validate_list::validate_list;
use crate::imp::json_to_rust::validation::validate_mut_list::validate_mut_list;
use crate::imp::json_to_rust::validation::validate_compatible::validate_compatible;
use crate::imp::json_to_rust::validation::validate_list_def::validate_list_def;

/// json読み出し時のチェックがあり、adjust時のチェックもあり、modifyインターフェース上のチェックもある。
/// それらでは補足しきれないチェックをするのがこれの役割。
///
/// ここではどこまでチェックするのか、補足しきれないものというのはどこまでか、というのが問題なのだけど、基本的には全部チェックする。他のチェックに依存しない。
/// というところを目指すことになるだろう。ダブルチェックや。
/// 型により可能な値はかなり限られるので、その中ですべてが妥当な値であるかどうかを調べることも出来ないことはないはず。
/// 基本的に
/// ・sabunがdefにあり、型があっていて、(optional)oldじゃないか
/// ・ref先が存在し、(optional)oldじゃないか
/// ・InnerListDefがListのDefにあり、InnerListがListItemにあるか。Root直下にInnerがないか。
/// ・Compatible指定されたリストが存在し、(optional)Oldでなく、Defが実際にCompatibleであるか
/// を確かめている
///
/// can_use_oldがあるとoldでも気にしなくなる。Jsonで初期値を読み込んだ後はcan_use_old=false,
/// 旧バージョンから以降した場合はcan_use_old=trueでやるとよかろうと思う
pub fn validate_root(root : &RootObject, can_use_old: bool) -> Result<()>{
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
                validate_list_def(data.default(), root, can_use_old, names)?;
                validate_data(data.default(), data.list(), root, can_use_old, names)?
            },
            RustValue::List(list) =>{
                validate_list_def(list.default(), root, can_use_old, names)?;
                validate_list(list.default(), list.list(), root, can_use_old, names)?
            },
            RustValue::Mut(m) =>{
                validate_compatible(m.default(), m.compatible(), root, can_use_old, names)?;
                validate_list_def(m.default(), root, can_use_old, names)?;
                validate_mut_list(m.default(), m.list(),  root, can_use_old, names)?
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