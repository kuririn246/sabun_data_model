use crate::imp::structs::value_type::ValueType;
use crate::imp::structs::rust_list::{ConstData, ConstList, MutList, InnerData, InnerList, InnerMutList};
use crate::imp::structs::root_value::RootValue;
use crate::imp::structs::list_value::{ListDefValue, ListSabValue};
use crate::imp::structs::rust_param::RustParam;
use crate::imp::structs::list_def_obj::ListDefObj;
use crate::imp::structs::inner_mut_def_obj::InnerMutDefObj;


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

#[repr(u64)]
#[derive(Debug, PartialEq, Clone)]
pub enum RustValueType{
    Bool, Num, Str, NumArray, StrArray,Num2Array,
    Data, List, Mut, InnerData, InnerList, InnerMut
}

impl RustValue{
    pub(crate) fn into_root_value(self) -> Result<RootValue, String>{
        let v = match self{
            RustValue::Param(p,v) => RootValue::Param(p,v),
            RustValue::Data(d) => RootValue::Data(d),
            RustValue::List(l) => RootValue::List(l),
            RustValue::Mut(m) => RootValue::Mut(m),
            _ =>{ return Err(self.type_string()); },
        };
        Ok(v)
    }

    pub(crate) fn into_root_value2(self, name : &str) -> crate::error::Result<RootValue>{
        match self.into_root_value(){
            Ok(a) => Ok(a),
            Err(s) =>{ Err(format!("{} the root obj can't have {}", name, s))? }
        }
    }

    pub(crate) fn into_list_def_value(self) -> Result<ListDefValue, String>{
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
    pub(crate) fn into_list_sab_value(self) -> Result<ListSabValue, String>{
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

