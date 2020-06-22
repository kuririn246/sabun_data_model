use crate::imp::structs::rust_param::RustParam;
use crate::imp::structs::rust_list::{InnerData, InnerList, InnerMutList};
use crate::imp::structs::rust_value::{RustValue, RustMemberType};
use crate::imp::structs::value_type::VarType;
use crate::imp::structs::qv::QvType;
use crate::imp::structs::list_def_obj::ListDefObj;
use crate::imp::structs::inner_mut_def_obj::InnerMutDefObj;

#[derive(Debug, PartialEq, Clone)]
pub enum ListDefValue{
    Param(RustParam, VarType),
    InnerDataDef(ListDefObj),
    InnerListDef(ListDefObj),
    InnerMutDef(InnerMutDefObj),
}

#[derive(Debug, PartialEq, Clone)]
pub enum ListSabValue{
    Param(RustParam),
    InnerData(InnerData),
    InnerList(InnerList),
    ///InnerMutListだけundefinedになりうる
    InnerMut(Option<InnerMutList>),
}

impl ListDefValue{
    pub(crate) fn into_rust_value(self) -> RustValue{
        match self{
            ListDefValue::Param(p,v) => RustValue::Param(p,v),
            ListDefValue::InnerDataDef(d) => RustValue::InnerDataDef(d),
            ListDefValue::InnerListDef(l) => RustValue::InnerListDef(l),
            ListDefValue::InnerMutDef(m) => RustValue::InnerMutDef(m),
        }
    }

    pub(crate) fn acceptable(&self, other : &ListSabValue) -> bool{
        if self.type_num() == other.type_num(){
            if self.value_type().acceptable(&other.qv_type()){
                return true;
            }
        }
        false
    }

    pub(crate) fn compatible(&self, other : &ListDefValue) -> bool{
        if self.type_num() == other.type_num(){
            if self.value_type().compatible(&other.value_type()){
                return true;
            }
        }
        false
    }


    pub(crate) fn value_type(&self) -> VarType {
        match self{
            ListDefValue::Param(_param, vt) => vt.clone(),
            ListDefValue::InnerMutDef(obj) => if obj.undefinable() { VarType::Undefiable } else{ VarType::Normal }
            _ => VarType::Normal,
        }
    }

    ///この数値は仮
    pub(crate) fn type_num(&self) -> RustMemberType {
        use RustMemberType::*;
        match self{
            ListDefValue::Param(param, _) => param.type_num(),
            ListDefValue::InnerDataDef(_) => InnerData,
            ListDefValue::InnerListDef(_) => InnerList,
            ListDefValue::InnerMutDef(_) => InnerMut,
        }
    }

    pub(crate) fn inner_def(&self) -> Option<&ListDefObj>{
        match self{
            ListDefValue::InnerDataDef(d) => Some(d),
            ListDefValue::InnerListDef(d) => Some(d),
            ListDefValue::InnerMutDef(obj) => Some(obj.list_def()),
            _ => None,
        }
    }
}

impl ListSabValue{
    ///この数値は仮
    pub(crate) fn type_num(&self) -> RustMemberType {
        use RustMemberType::*;

        match self{
            ListSabValue::Param(param) => param.type_num(),
            ListSabValue::InnerData(_) => InnerData,
            ListSabValue::InnerList(_) => InnerList,
            ListSabValue::InnerMut(_) => InnerMut,
        }
    }

    ///ValueType::NormalとしてRustValue化する。これをjsonにすると、param_name : ["Num",null]とか言った感じになって、
    /// nullなのに?がない形だが、ListSabでは名前に?をつけるのは必須ではなく、むしろノイズになるので?は消す方が良いのでこうする
    /// nullの場合は"param_name?":["Num",null]のように?を補う実装があってもいいとは思うが、使いみちが今の所ない
    pub(crate) fn into_rust_value_for_json(self) -> RustValue{
        match self{
            //value側は名前に?とか!とかつけなくてよいのでValueType::Normal
            ListSabValue::Param(p) => RustValue::Param(p, VarType::Normal),
            ListSabValue::InnerData(d) => RustValue::InnerData(d),
            ListSabValue::InnerList(l) => RustValue::InnerList(l),
            ListSabValue::InnerMut(m) => RustValue::InnerMut(m),
        }
    }

    pub(crate) fn qv_type(&self) -> QvType{
        match self{
            ListSabValue::Param(p) => p.qv_type(),
            ListSabValue::InnerMut(m) => if m.is_some(){ QvType::Val } else{ QvType::Undefined },
            _ => QvType::Val,
        }
    }
}