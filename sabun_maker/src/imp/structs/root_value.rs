use crate::imp::structs::rust_param::RustParam;
use crate::imp::structs::var_type::VarType;
use crate::imp::structs::rust_list::{ConstData, ConstList, MutList};
use crate::imp::structs::rust_value::RustValue;
use crate::imp::structs::list_def_obj::ListDefObj;

#[derive(Debug, PartialEq, Clone)]
pub enum RootValue{
    Param(RustParam, VarType),
    Data(ConstData),
    List(ConstList),
    Mut(MutList),
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


    pub(crate) fn into_rust_value(self) -> RustValue{
        match self{
            RootValue::Param(p,v) => RustValue::Param(p,v),
            RootValue::Data(d) => RustValue::Data(d),
            RootValue::List(l) => RustValue::List(l),
            RootValue::Mut(m) => RustValue::Mut(m),
        }
    }
}
