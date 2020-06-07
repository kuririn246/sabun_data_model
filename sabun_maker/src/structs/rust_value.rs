use crate::structs::value_type::ValueType;
use crate::structs::qv::{QvType, Qv};
use crate::structs::rust_list::{ConstData, ConstList, MutList, InnerList, InnerData, InnerMutList};
use crate::structs::array_type::ArrayType;
use crate::structs::root_object::{ListDefObj, InnerMutDefObj};
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum RustParam{
    Bool(Qv<bool>),
    Number(Qv<f64>),
    String(Qv<RustString>),
    Array(RustArray),
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum RustValue{
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
    pub(crate) fn to_root_value(self) -> Result<RootValue, String>{
        let v = match self{
            RustValue::Param(p,v) => RootValue::Param(p,v),
            RustValue::Data(d) => RootValue::Data(d),
            RustValue::List(l) => RootValue::List(l),
            RustValue::Mut(m) => RootValue::Mut(m),
            _ =>{ return Err(self.type_string()); },
        };
        Ok(v)
    }

    pub(crate) fn to_root_value2(self, name : &str) -> crate::error::Result<RootValue>{
        match self.to_root_value(){
            Ok(a) => Ok(a),
            Err(s) =>{ Err(format!("{} the root obj can't have {}", name, s))? }
        }
    }

    pub(crate) fn to_list_def_value(self) -> Result<ListDefValue, String>{
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
    pub(crate) fn to_list_sab_value(self) -> Result<ListSabValue, String>{
        let v = match self{
            RustValue::Param(p,_v) => ListSabValue::Param(p),
            RustValue::InnerData(d) => ListSabValue::InnerData(d),
            RustValue::InnerList(l) => ListSabValue::InnerList(l),
            RustValue::InnerMut(m) => ListSabValue::InnerMut(m),
            _ =>{ return Err(self.type_string()); },
        };
        Ok(v)
    }

    pub(crate) fn type_string(&self) -> String{
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
pub(crate) enum RootValue{
    Param(RustParam, ValueType),
    Data(ConstData),
    List(ConstList),
    Mut(MutList),
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum ListDefValue{
    Param(RustParam, ValueType),
    InnerDataDef(ListDefObj),
    InnerListDef(ListDefObj),
    InnerMutDef(InnerMutDefObj),
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum ListSabValue{
    Param(RustParam),
    InnerData(InnerData),
    InnerList(InnerList),
    ///InnerMutListだけundefinedになりうる
    InnerMut(Option<InnerMutList>),
}


pub(crate) enum ListType{
    Data, List, Mut, InnerData, InnerList, InnerMut, InnderDataDef, InnerListDef, //InnerMutDef,
}



#[derive(Debug, PartialEq, Clone)]
pub(crate) struct RustArray{
    array : Box<RustArrayInternal>,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct RustArrayInternal{
    qv : Qv<Vec<RustParam>>,
    at : ArrayType,
}

impl RustArray{
    pub(crate) fn new(qv : Qv<Vec<RustParam>>, at : ArrayType) -> RustArray{
        RustArray{ array : Box::new(RustArrayInternal{
            qv, at
        })}
    }
    pub(crate) fn null(at : ArrayType) -> RustArray{
        RustArray{ array : Box::new(RustArrayInternal{
            qv : Qv::Null, at
        })}
    }
    pub(crate) fn undefined(at : ArrayType) -> RustArray{
        RustArray{ array : Box::new(RustArrayInternal{
            qv : Qv::Undefined, at
        })}
    }
    pub(crate) fn qv(&self) -> &Qv<Vec<RustParam>>{ &self.array.qv }
    pub(crate) fn array_type(&self) -> ArrayType{ self.array.at.clone() }
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct RustString{
    str : Box<String>,
}

impl RustString{
    pub(crate) fn new(s : String) -> RustString{ RustString{ str : Box::new(s) }}
    pub(crate) fn str(&self) -> &str{ self.str.as_ref().as_ref() }
}

impl Display for RustString{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.str().fmt(f)
    }
}


impl RustParam{
    pub(crate) fn qv_type(&self) -> QvType {
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
            RustParam::Array(a) => match a.array_type(){
                ArrayType::Num => 3,
                ArrayType::String => 4,
                ArrayType::Num2 => 5,
            },
        }
    }

    pub(crate) fn acceptable(&self, other : &Self) -> bool {
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
    pub(crate) fn to_undefined(&self) -> Self{
        match self{
            RustParam::Bool(_) => RustParam::Bool(Qv::Undefined),
            RustParam::Number(_) => RustParam::Number(Qv::Undefined),
            RustParam::String(_) => RustParam::String(Qv::Undefined),
            RustParam::Array(a) => RustParam::Array(RustArray::undefined(a.array_type()))

        }
    }
}



impl ListDefValue{
    pub(crate) fn to_rust_value(self) -> RustValue{
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


    pub(crate) fn value_type(&self) -> ValueType{
        match self{
            ListDefValue::Param(_param, vt) => vt.clone(),
            ListDefValue::InnerMutDef(obj) => if obj.undefinable() { ValueType::Undefiable } else{ ValueType::Normal }
            _ => ValueType::Normal,
        }
    }

    ///この数値は仮
    pub(crate) fn type_num(&self) -> usize{
        match self{
            ListDefValue::Param(param, _) => match param{
                RustParam::Bool(_) => 0,
                RustParam::Number(_) => 1,
                RustParam::String(_) => 2,
                RustParam::Array(a) => match a.array_type(){
                    ArrayType::Num => 3,
                    ArrayType::String => 4,
                    ArrayType::Num2 => 5,
                },
            },
            ListDefValue::InnerListDef(_) => 6,
            ListDefValue::InnerDataDef(_) => 7,
            ListDefValue::InnerMutDef(_) => 8,
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
    pub(crate) fn type_num(&self) -> usize{
        match self{
            ListSabValue::Param(param) => match param{
                RustParam::Bool(_) => 0,
                RustParam::Number(_) => 1,
                RustParam::String(_) => 2,
                RustParam::Array(a) => match a.array_type(){
                    ArrayType::Num => 3,
                    ArrayType::String => 4,
                    ArrayType::Num2 => 5,
                },
            },
            ListSabValue::InnerList(_) => 6,
            ListSabValue::InnerData(_) => 7,
            ListSabValue::InnerMut(_) => 8,
        }
    }

    pub(crate) fn to_rust_value(self) -> RustValue{
        match self{
            //value側は名前に?とか!とかつけなくてよいのでValueType::Normal
            ListSabValue::Param(p) => RustValue::Param(p, ValueType::Normal),
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

impl RootValue{
    pub(crate) fn list_def(&self) ->  Option<&ListDefObj> {
        match self {
            RootValue::Data(d) => Some(d.default()),
            RootValue::List(d) => Some(d.default()),
            RootValue::Mut(d) => Some(d.default()),
            _ => None,
        }
    }


    pub(crate) fn to_rust_value(self) -> RustValue{
        match self{
            RootValue::Param(p,v) => RustValue::Param(p,v),
            RootValue::Data(d) => RustValue::Data(d),
            RootValue::List(l) => RustValue::List(l),
            RootValue::Mut(m) => RustValue::Mut(m),
        }
    }
}
