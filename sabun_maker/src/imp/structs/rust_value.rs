use crate::imp::structs::var_type::VarType;
use crate::imp::structs::rust_list::{ConstTable, ConstTemplate, MutList, InnerTemplate, InnerMutList};
use crate::imp::structs::root_value::RootValue;
use crate::imp::structs::list_value::{ListDefValue, ListSabValue};
use crate::imp::structs::rust_param::RustParam;
use crate::imp::structs::list_def_obj::ListDefObj;
use crate::imp::structs::inner_mut_def_obj::InnerMutDefObj;


#[derive(Debug, PartialEq, Clone)]
pub enum RustValue{
    Param(RustParam, VarType),
    Table(ConstTable),
    Template(ConstTemplate),
    MutList(MutList),
    //InnerData(InnerData),
    InnerTemp(InnerTemplate),
    ///InnerMutListだけundefinedになりうる
    InnerMut(Option<InnerMutList>),
    //InnerDataDef(ListDefObj),
    InnerTempDef(ListDefObj),
    InnerMutDef(InnerMutDefObj),
}

#[repr(u64)]
#[derive(Debug, PartialEq, Clone)]
pub enum RustMemberType {
    Bool, Float, Int, Str, IntArray, FloatArray,
    Table, Template, MutList,
    InnerTemp, InnerMut
}

impl RustValue{
    pub(crate) fn into_root_value(self) -> Result<RootValue, String>{
        let v = match self{
            RustValue::Param(p,v) => RootValue::Param(p,v),
            RustValue::Table(d) => RootValue::Table(d),
            RustValue::Template(l) => RootValue::Template(l),
            RustValue::MutList(m) => RootValue::List(m),
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
            //RustValue::InnerDataDef(d) => ListDefValue::InnerDataDef(d),
            RustValue::InnerTempDef(l) => ListDefValue::InnerTempDef(l),
            RustValue::InnerMutDef(m) => ListDefValue::InnerMutDef(m),
            _ =>{ return Err(self.type_string()); },
        };
        Ok(v)
    }

    ///失敗時はtype_stringを返す
    pub(crate) fn into_list_sab_value(self) -> Result<ListSabValue, String>{
        let v = match self{
            RustValue::Param(p,_v) => ListSabValue::Param(p),
            //RustValue::InnerData(d) => ListSabValue::InnerData(d),
            RustValue::InnerTemp(l) => ListSabValue::InnerTemp(l),
            RustValue::InnerMut(m) => ListSabValue::InnerMut(m),
            _ =>{ return Err(self.type_string()); },
        };
        Ok(v)
    }

    pub(crate) fn type_string(&self) -> String{
        let s = match self{
            RustValue::Param(_, _) => "Param",
            RustValue::Table(_) => "Table",
            RustValue::Template(_) => "Template",
            RustValue::MutList(_) => "MutList",
           // RustValue::InnerData(_) => "InnerData",
            RustValue::InnerTemp(_) => "InnerTemp",
            RustValue::InnerMut(_) => "InnerMut",
            RustValue::InnerTempDef(_) => "InnerTempDef",
            RustValue::InnerMutDef(_) => "InnerMutDef",
        };
        s.to_string()
    }
}

