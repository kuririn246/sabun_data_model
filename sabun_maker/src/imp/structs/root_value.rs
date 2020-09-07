use crate::imp::structs::rust_param::RustParam;
use crate::imp::structs::var_type::VarType;
use crate::imp::structs::rust_list::{ConstTable, ConstTemplate, MutList};
use crate::imp::structs::rust_value::RustValue;
use crate::imp::structs::list_def_obj::ListDefObj;

#[derive(Debug, PartialEq, Clone)]
pub enum RootValue{
    Param(RustParam, VarType),
    Table(ConstTable),
    Template(ConstTemplate),
    List(MutList),
}

impl RootValue{
    pub(crate) fn list_def(&self) ->  Option<&ListDefObj> {
        match self {
            RootValue::Table(d) => Some(d.default()),
            RootValue::Template(d) => Some(d.default()),
            RootValue::List(d) => Some(d.default()),
            _ => None,
        }
    }


    pub(crate) fn into_rust_value(self) -> RustValue{
        match self{
            RootValue::Param(p,v) => RustValue::Param(p,v),
            RootValue::Table(d) => RustValue::Table(d),
            RootValue::Template(l) => RustValue::Template(l),
            RootValue::List(m) => RustValue::MutList(m),
        }
    }
}
