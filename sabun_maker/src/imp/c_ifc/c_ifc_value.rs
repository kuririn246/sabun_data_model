use crate::imp::structs::root_obj::RootObject;
use std::ffi::CStr;
use crate::imp::c_ifc::param_ifc::{ParamBool, ParamNum, ParamStr, ParamNumArray, ParamStrArray, ParamNum2Array, ParamInf};

pub enum CifcValue{
    Bool(ParamInf<bool>),
    Num(ParamInf<f64>),
    Str(ParamInf<*const String>),
    NumArray(ParamInf<*const Vec<f64>>),
    StrArray(ParamInf<*const Vec<String>>),
    Num2Array(ParamInf<*const Vec<Vec<f64>>>),

}

