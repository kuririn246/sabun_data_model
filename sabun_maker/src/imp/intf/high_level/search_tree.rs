use crate::imp::structs::root_obj::RootObject;
use crate::imp::structs::qv::{Qv, QvType};
use crate::imp::structs::root_value::RootValue;
use crate::imp::structs::rust_list::{ConstData, ConstList, MutList};
use crate::imp::structs::rust_param::RustParam;
use crate::imp::structs::value_type::ValueType;
use crate::imp::structs::rust_value::RustValueType;
use crate::imp::structs::array_type::ArrayType;
use crate::imp::structs::util::set_sabun::SetSabunError;
use crate::imp::structs::rust_string::RustString;
use crate::imp::structs::rust_array::RustArray;

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
    GetLen, GetIndexes, GetIDs, GetMemberDesc, GetRefDesc, //未実装 GetUnsafePtr,
    GetBool, GetNum, GetStr, GetNumArray, GetStrArray, GetNum2Array,
    Set(SetParam)
}

#[derive(Debug, Clone, PartialEq)]
pub enum SetParam{
    Bool(Qv<bool>), Num(Qv<f64>), Str(Qv<String>), NumArray(Qv<Vec<f64>>), StrArray(Qv<Vec<String>>), Num2Array(Qv<Vec<Vec<f64>>>),
}

impl SetParam{
    pub fn to_rust_param(&self) -> RustParam{
        match self{
            SetParam::Bool(b) => RustParam::Bool(b.clone()),
            SetParam::Num(b) => RustParam::Number(b.clone()),
            SetParam::Str(b) => RustParam::String(b.map(|s| RustString::new(s.to_string()))),
            SetParam::NumArray(b) => RustParam::Array(RustArray::from_num_array(b), ArrayType::Num),
            SetParam::StrArray(b) => RustParam::Array(RustArray::from_str_array(b), ArrayType::String),
            SetParam::Num2Array(b) => RustParam::Array(RustArray::from_num2_array(b), ArrayType::Num2),
        }
    }
}

pub enum ActionType{ GetParam, SetParam, Other }

impl Action{
    pub(crate) fn is_get_param(&self) -> bool{
        use Action::*;

        match self{
            GetBool | GetNum | GetStr | GetNumArray | GetStrArray | GetNum2Array => true,
            _ => false,
        }
    }

    pub(crate) fn param_type_num(&self) -> Option<RustValueType>{
        use Action::*;
        use RustValueType::*;

        let t = match self{
            GetBool | Set(SetParam::Bool(_))=> Bool,
            GetNum | Set(SetParam::Num(_)) => Num,
            GetStr | Set(SetParam::Str(_)) => Str,
            GetNumArray | Set(SetParam::NumArray(_)) => NumArray,
            GetStrArray | Set(SetParam::StrArray(_)) => StrArray,
            GetNum2Array | Set(SetParam::Num2Array(_)) => Num2Array,
            _ =>{ return None; }
        };
        return Some(t);
    }

}

pub enum ActionResult{
    Len(usize), Indexes(Option<Vec<u64>>), IDs(Vec<String>), MemberDesc(Vec<MemberDesc>), RefDesc(Vec<RefDesc>),
    Bool(Qv<bool>), Num(Qv<f64>), Str(Qv<String>), NumArray(Qv<Vec<f64>>), StrArray(Qv<Vec<String>>), Num2Array(Qv<Vec<Vec<f64>>>),
    SetOk,
}

pub struct ActionError{
    pub index : ActionErrorIndex,
    pub et : AET,
}

impl ActionError{
    pub fn member(index : usize, et : AET) -> ActionError{ ActionError{ index : ActionErrorIndex::Member(index), et } }
    pub fn from_col(index : usize, et : AET) -> ActionError{ ActionError{ index : ActionErrorIndex::FromCol(index), et } }
    pub fn action(et : AET) -> ActionError{ ActionError{ index : ActionErrorIndex::Action, et } }
}

pub enum ActionErrorIndex{
    Member(usize), FromCol(usize), Action
}

///Action Error Type
pub enum AET{
    IndexOutOfRange, MemberNotFound,
    ///The action couldn't be applied to the item
    TypeMismatch,
    ///Set Null(Undefined) into Non-Null(Non-Undefined) variable
    QvTypeMismatch,
    ParamTypeMismatch,
}

pub struct MemberDesc{
    pub member : String,
    pub qv_type : QvType,
    pub member_type : MemberType,
    pub is_old : bool,
}

pub struct RefDesc{
    pub member : String,
    pub qv_type : QvType,
    pub is_old : bool,
}

pub enum MemberType{
    Bool, Num, Str, NumArray, StrArray, NumArray2,
    Data, List, MutList, InnerData, InnerList, InnerMutList,
}

///低速だが、速度を求めるなら if(a == null){ a = search_from_root(hoge); a } else { a }で読み出すような形になるだろう
///それでも起動直後の速度を求めるなら、このインターフェースは不十分か？
/// しかしまあ、E*N回のハッシュテーブルアクセスが問題になるなら、それよりもハッシュテーブルのコンストラクトにかかる時間の方が問題になるはずだ
/// 仮にハッシュテーブルアクセスが頻発するのが問題になるユースケースなら上の方式を使えば速くなるだろう
pub fn search_from_root(root : &mut RootObject, args : &SearchArgs) -> Result<ActionResult, ActionError> {
    if args.member_name.len() == 0 {
        return if let Action::GetMemberDesc = args.action {
            Ok(ActionResult::MemberDesc(get_root_mem_desc(root)))
        } else {
            Err(ActionError::action(AET::TypeMismatch))
        }
    }
    let name = &args.member_name[0];
    let val = if let Some(h) = root.default().get(name) { h } else {
        return Err(ActionError::member(0, AET::MemberNotFound));
    };

    return match val {
        RootValue::Param(p, _vt) => {
            if args.action.is_get_param() {

                return get_param(&args.action, p, root.sabun().get(name));
            }
            if let Action::Set(param) = &args.action {
                match root.set_sabun(name.to_string(), param.to_rust_param()){
                    Ok(_) =>{ return Ok(ActionResult::SetOk); },
                    Err(e) =>{
                        match e {
                            SetSabunError::ParamNotFound => unreachable!(),
                            SetSabunError::QvTypeMismatch =>{ return Err(ActionError::action(AET::QvTypeMismatch)); },
                            SetSabunError::ParamTypeMismatch =>{ return Err(ActionError::action(AET::ParamTypeMismatch)); }
                        }
                    }
                }
            }
            //Paramに対してはGetかSetしかできましぇん
            return Err(ActionError::action(AET::TypeMismatch))
        },
        RootValue::Data(d) => search_from_data(d, args, 0),
        RootValue::List(l) => search_from_list(l, args, 0),
        RootValue::Mut(m) => search_from_mut(m, args, 0),
    };
}

pub fn get_param(action : &Action, p : &RustParam, sab : Option<&RustParam>) -> Result<ActionResult, ActionError>{
    let t = if let Some(t) = action.param_type_num() { t } else{
        return Err(ActionError::action(AET::TypeMismatch));
    };
    if t != p.type_num(){
        return Err(ActionError::action(AET::ParamTypeMismatch));
    }
    if let Some(sab) = sab{
        //これはつまりSabunとDefaultの型が違うということであり、あってはならぬこと
        if t != sab.type_num(){ unreachable!() }
        return Ok(get_action_param_uncheck(action, sab));
    } else{
        return Ok(get_action_param_uncheck(action, p));
    }
}

pub fn get_action_param_uncheck(action : &Action, p : &RustParam) -> ActionResult{
    match action{
        Action::GetBool => if let RustParam::Bool(a) = p{ ActionResult::Bool(a.clone()) } else{ unreachable!() },
        Action::GetNum => if let RustParam::Number(a) = p{ ActionResult::Num(a.clone()) } else{ unreachable!() },
        Action::GetStr => if let RustParam::String(a) = p{ ActionResult::Str(a.map(|s| s.str().to_string())) } else{ unreachable!() },
        Action::GetNumArray => if let RustParam::Array(a, at) = p{ ActionResult::NumArray(a.to_num_array().unwrap()) } else{ unreachable!() },
        Action::GetStrArray => if let RustParam::Array(a, at) = p{ ActionResult::StrArray(a.to_str_array().unwrap()) } else{ unreachable!() },
        Action::GetNum2Array => if let RustParam::Array(a, at) = p{ ActionResult::Num2Array(a.to_num2_array().unwrap()) } else{ unreachable!() },
        _ =>{ unreachable!() },
    }
}



pub fn get_root_mem_desc(root : &RootObject) -> Vec<MemberDesc>{
    todo!()
}

pub fn search_from_data(data : &ConstData, args : &SearchArgs, index : usize) -> Result<ActionResult, ActionError>{
    todo!()
}

pub fn search_from_list(list : &ConstList, args : &SearchArgs, index : usize) -> Result<ActionResult, ActionError>{
    todo!()
}

pub fn search_from_mut(list : &MutList, args : &SearchArgs, index : usize) -> Result<ActionResult, ActionError>{
    todo!()
}