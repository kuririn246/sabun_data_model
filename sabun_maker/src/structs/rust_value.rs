use crate::structs::value_type::ValueType;
use crate::structs::qv::{QvType, Qv};
use crate::structs::rust_list::{ConstData, ConstList, MutList, InnerList, InnerData, InnerMutList};
use crate::structs::array_type::ArrayType;
use crate::structs::root_object::{ListDefObj, InnerMutDefObj};
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum RustParam{
    Bool(Qv<bool>),
    Number(Qv<f64>),
    String(Qv<RustString>),
    Array(RustArray),
}

#[derive(Debug, PartialEq, Clone)]
pub enum RustValue{
    Param(RustParam, ValueType),
    Data(ConstData),
    List(ConstList),
    Mut(MutList),
    InnerData(InnerData),
    InnerList(InnerList),
    ///InnerMutListだけundefinedになりうる
    InnerMut(Option<InnerMutList>),
    InnerDataDef(ListDefObj),
    InnerListDef(ListDefObj),
    InnerMutDef(InnerMutDefObj),
}

impl RustValue{
    pub fn to_root_value(self) -> Result<RootValue, String>{
        let v = match self{
            RustValue::Param(p,v) => RootValue::Param(p,v),
            RustValue::Data(d) => RootValue::Data(d),
            RustValue::List(l) => RootValue::List(l),
            RustValue::Mut(m) => RootValue::Mut(m),
            _ =>{ return Err(self.type_string()); },
        };
        Ok(v)
    }

    pub fn to_root_value2(self, name : &str) -> crate::error::Result<RootValue>{
        match self.to_root_value(){
            Ok(a) => Ok(a),
            Err(s) =>{ Err(format!("{} the root obj can't have {}", name, s))? }
        }
    }

    pub fn to_list_def_value(self) -> Result<ListDefValue, String>{
        let v = match self{
            RustValue::Param(p,v) => ListDefValue::Param(p,v),
            RustValue::InnerDataDef(d) => ListDefValue::InnerDataDef(d),
            RustValue::InnerListDef(l) => ListDefValue::InnerListDef(l),
            RustValue::InnerMutDef(m) => ListDefValue::InnerMutDef(m),
            _ =>{ return Err(self.type_string()); },
        };
        Ok(v)
    }

    ///失敗時はtype_stringを返す
    pub fn to_list_sab_value(self) -> Result<ListSabValue, String>{
        let v = match self{
            RustValue::Param(p,v) => ListSabValue::Param(p),
            RustValue::InnerData(d) => ListSabValue::InnerData(d),
            RustValue::InnerList(l) => ListSabValue::InnerList(l),
            RustValue::InnerMut(m) => ListSabValue::InnerMut(m),
            _ =>{ return Err(self.type_string()); },
        };
        Ok(v)
    }

    pub fn type_string(&self) -> String{
        let s = match self{
            RustValue::Param(_, _) => "Param",
            RustValue::Data(_) => "Data",
            RustValue::List(_) => "List",
            RustValue::Mut(_) => "Mut",
            RustValue::InnerData(_) => "InnerData",
            RustValue::InnerList(_) => "InnerList",
            RustValue::InnerMut(_) => "InnerMut",
            RustValue::InnerDataDef(_) => "InnerDataDef",
            RustValue::InnerListDef(_) => "InnerListDef",
            RustValue::InnerMutDef(_) => "InnerMutDef",
        };
        s.to_string()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum RootValue{
    Param(RustParam, ValueType),
    Data(ConstData),
    List(ConstList),
    Mut(MutList),
}

#[derive(Debug, PartialEq, Clone)]
pub enum ListDefValue{
    Param(RustParam, ValueType),
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


pub enum ListType{
    Data, List, Mut, InnerData, InnerList, InnerMut, InnderDataDef, InnerListDef, InnerMutDef,
}

pub enum RustValueKind {
    Param, InnerDef, InnerList, List
}

impl RustValueKind {
    pub fn acceptable(&self, other : &Self) -> bool{
        match self{
            RustValueKind::Param => match other{
                RustValueKind::Param => true,
                _ => false,
            }
            RustValueKind::InnerDef => match other{
                RustValueKind::InnerList => true,
                _ => false,
            }
            RustValueKind::List | RustValueKind::InnerList => false,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct RustArray{
    array : Box<RustArrayInternal>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct RustArrayInternal{
    qv : Qv<Vec<RustParam>>,
    at : ArrayType,
}

impl RustArray{
    pub fn new(qv : Qv<Vec<RustParam>>, at : ArrayType) -> RustArray{
        RustArray{ array : Box::new(RustArrayInternal{
            qv, at
        })}
    }
    pub fn null(at : ArrayType) -> RustArray{
        RustArray{ array : Box::new(RustArrayInternal{
            qv : Qv::Null, at
        })}
    }
    pub fn undefined(at : ArrayType) -> RustArray{
        RustArray{ array : Box::new(RustArrayInternal{
            qv : Qv::Undefined, at
        })}
    }
    pub fn qv(&self) -> &Qv<Vec<RustParam>>{ &self.array.qv }
    pub fn array_type(&self) -> ArrayType{ self.array.at.clone() }
}

#[derive(Debug, PartialEq, Clone)]
pub struct RustString{
    str : Box<String>,
}

impl RustString{
    pub fn new(s : String) -> RustString{ RustString{ str : Box::new(s) }}
    pub fn str(&self) -> &str{ self.str.as_ref().as_ref() }
}

impl Display for RustString{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.str().fmt(f)
    }
}


impl RustParam{
    pub fn qv_type(&self) -> QvType {
        match self {
            RustParam::Bool(b) => b.qv_type(),
            RustParam::Number(n) => n.qv_type(),
            RustParam::String(s) => s.qv_type(),
            RustParam::Array(a) => a.qv().qv_type(),
        }
    }

    pub(crate) fn type_num(&self) -> usize{
        match self{
            RustParam::Bool(_) => 0,
            RustParam::Number(_) => 1,
            RustParam::String(_) => 2,
            RustParam::Array(_) => 3,
        }
    }

    pub fn acceptable(&self, other : &Self) -> bool {
        if self.type_num() != other.type_num() {
            return false;
        }
        if let RustParam::Array(s) = self {
            if let RustParam::Array(o) = other {
                //array_typeが一致してるかはここまでしないと調べられないだろうか・・・？
                if s.array_type().type_num() != o.array_type().type_num() {
                    return false;
                }
            } else { unreachable!() }
        }
        return true;
    }

    ///型情報を維持したままundefinedに変える
    pub fn to_undefined(&self) -> Self{
        match self{
            RustParam::Bool(_) => RustParam::Bool(Qv::Undefined),
            RustParam::Number(_) => RustParam::Number(Qv::Undefined),
            RustParam::String(_) => RustParam::String(Qv::Undefined),
            RustParam::Array(a) => RustParam::Array(RustArray::undefined(a.array_type()))

        }
    }
}

impl RustValue{

    pub fn is_param(&self) -> bool{
        self.type_num() <= 3
    }

    pub fn value_type(&self) -> ValueType{
        match self{
            RustValue::Param(_param, vt) => vt.clone(),
            RustValue::InnerMutDef(obj) => if obj.undefinable() { ValueType::Undefiable } else{ ValueType::Normal }
            _ => ValueType::Normal,
        }
    }

    pub fn list_type(&self) -> Option<ListType>{
        Some(match self{
            RustValue::Data(_) => ListType::Data,
            RustValue::List(_) => ListType::List,
            RustValue::Mut(_) => ListType::Mut,
            RustValue::InnerData(_) => ListType::InnerData,
            RustValue::InnerList(_) => ListType::InnerList,
            RustValue::InnerMut(_) => ListType::InnerMut,
            RustValue::InnerDataDef(_) => ListType::InnderDataDef,
            RustValue::InnerListDef(_) => ListType::InnerListDef,
            RustValue::InnerMutDef(_) => ListType::InnerMutDef,
            _ => return None,
        })
    }

    pub fn qv_type(&self) -> QvType{
        match self{
            RustValue::Param(p, _) => p.qv_type(),
            RustValue::InnerMut(b) => if b.is_some(){ QvType::Val } else{ QvType::Undefined },
            _ =>{ QvType::Val }
        }
    }

    pub fn value_kind(&self) -> RustValueKind {
        match self{
            RustValue::Param(_,_) => RustValueKind::Param,
            RustValue::Data(_) | RustValue::List(_) | RustValue::Mut(_) => RustValueKind::List,
            RustValue::InnerData(_) | RustValue::InnerList(_) | RustValue::InnerMut(_) => RustValueKind::InnerList,
            RustValue::InnerDataDef(_) |RustValue::InnerListDef(_) |RustValue::InnerMutDef(_) => RustValueKind::InnerDef,
        }
    }

    pub fn inner_def(&self) -> Option<&ListDefObj>{
        match self{
            RustValue::InnerDataDef(d) => Some(d),
            RustValue::InnerListDef(d) => Some(d),
            RustValue::InnerMutDef(obj) => Some(obj.list_def()),
            _ => None,
        }
    }

    pub fn list_def(&self) ->  Option<&ListDefObj> {
        match self {
            RustValue::Data(d) => Some(d.default()),
            RustValue::List(d) => Some(d.default()),
            RustValue::Mut(d) => Some(d.default()),
            RustValue::InnerDataDef(d) => Some(d),
            RustValue::InnerListDef(d) => Some(d),
            RustValue::InnerMutDef(obj) => Some(obj.list_def()),
            _ => None,
        }
    }

    ///defaultとsabun, list_defとlist_item sabunのような時に、defaultの変化値としてsabunが適当かどうか
    ///調べるのは型だけで、listの中身がちゃんとdefaultと整合してるかまでは調べてくれない
    pub fn acceptable(&self, other : &Self) -> bool {
        if self.type_num() != other.type_num() {
            return false;
        }
        if self.value_type().acceptable(&other.qv_type()) == false {
            return false;
        }
        if let RustValue::Param(sp,_) = self {
            if let RustValue::Param(op, _) = other {
                return sp.acceptable(op)
            } else { unreachable!() }
        }
        if self.value_kind().acceptable(&other.value_kind()) == false {
            return false;
        }
        return true;
    }
}

impl ListDefValue{
    pub fn to_rust_value(self) -> RustValue{
        match self{
            ListDefValue::Param(p,v) => RustValue::Param(p,v),
            ListDefValue::InnerDataDef(d) => RustValue::InnerDataDef(d),
            ListDefValue::InnerListDef(l) => RustValue::InnerListDef(l),
            ListDefValue::InnerMutDef(m) => RustValue::InnerMutDef(m),
        }
    }

    pub fn acceptable(&self, other : &ListSabValue) -> bool{

    }

    ///この数値は仮
    pub(crate) fn type_num(&self) -> usize{
        match self{
            ListDefValue::Param(param, _) => match param{
                RustParam::Bool(_) => 0,
                RustParam::Number(_) => 1,
                RustParam::String(_) => 2,
                RustParam::Array(_) => 3,
            },
            ListDefValue::InnerListDef(_) => 7,
            ListDefValue::InnerDataDef(_) => 8,
            ListDefValue::InnerMutDef(_) => 9,
        }
    }

    pub fn inner_def(&self) -> Option<&ListDefObj>{
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
    pub(crate) fn type_num(&self) -> usize{
        match self{
            ListSabValue::Param(param) => match param{
                RustParam::Bool(_) => 0,
                RustParam::Number(_) => 1,
                RustParam::String(_) => 2,
                RustParam::Array(_) => 3,
            },
            ListSabValue::InnerList(_) => 7,
            ListSabValue::InnerData(_) => 8,
            ListSabValue::InnerMut(_) => 9,
        }
    }
}

impl RootValue{
    pub fn list_def(&self) ->  Option<&ListDefObj> {
        match self {
            RustValue::Data(d) => Some(d.default()),
            RustValue::List(d) => Some(d.default()),
            RustValue::Mut(d) => Some(d.default()),
            _ => None,
        }
    }

    pub fn value_type(&self) -> ValueType{
        match self{
            RootValue::Param(_param, vt) => vt.clone(),
            //RustValue::InnerMutDef(obj) => if obj.undefinable() { ValueType::Undefiable } else{ ValueType::Normal }
            _ => ValueType::Normal,
        }
    }

    pub fn to_rust_value(self) -> RustValue{
        match self{
            RootValue::Param(p,v) => RustValue::Param(p,v),
            RootValue::Data(d) => RustValue::Data(d),
            RootValue::List(l) => RustValue::List(l),
            RootValue::Mut(m) => RustValue::Mut(m),
        }
    }
}
