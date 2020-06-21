use crate::imp::structs::qv::Qv;
use crate::imp::structs::rust_param::RustParam;
use crate::imp::structs::rust_string::RustString;
use crate::imp::structs::array_type::ArrayType;

#[derive(Debug, PartialEq, Clone)]
pub struct RustArray{
    array : Box<Qv<Vec<RustParam>>>,
}

impl RustArray{
    pub(crate) fn new(qv : Qv<Vec<RustParam>>) -> RustArray{
        RustArray{ array : Box::new(qv) }
    }

    pub(crate) fn from_num_array(qv : &Qv<RustNumArray>) -> RustArray{
        RustArray::new(qv.map(|a| a.to_params()))
    }
    pub(crate) fn from_str_array(qv : &Qv<RustStrArray>) -> RustArray{
        RustArray::new(qv.map(|a| a.to_params()))
    }
    pub(crate) fn from_num2_array(qv : &Qv<RustNum2Array>) -> RustArray{
        RustArray::new(qv.map(|a| a.to_params()))
    }

    pub(crate) fn qv(&self) -> &Qv<Vec<RustParam>>{ self.array.as_ref() }

    pub(crate) fn to_num_array(&self) -> Option<Qv<RustNumArray>>{
        self.qv().opt_map(|a| RustNumArray::from_params(a))
    }

    pub(crate) fn to_str_array(&self) -> Option<Qv<RustStrArray>>{
        self.qv().opt_map(|a| RustStrArray::from_params(a))
    }

    pub(crate) fn to_num2_array(&self) -> Option<Qv<RustNum2Array>>{
        self.qv().opt_map(|a| RustNum2Array::from_params(a))
    }

    pub(crate) fn to_param(&self, at : &ArrayType) -> Option<RustParam>{
        Some(match at{
            ArrayType::Num =>{ RustParam::NumArray(self.to_num_array()?) },
            ArrayType::String =>{ RustParam::StrArray(self.to_str_array()?)}
            ArrayType::Num2 =>{ RustParam::Num2Array(self.to_num2_array()?)}
        })
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct RustNumArray{
    b : Box<Vec<f64>>,
}

impl RustNumArray{
    pub(crate) fn new(b : Vec<f64>) -> RustNumArray{ RustNumArray{ b : Box::new(b) }}
    pub(crate) fn as_ref(&self) -> &Vec<f64>{ self.b.as_ref() }
    pub(crate) fn to_params(&self) -> Vec<RustParam>{
        self.b.iter().map(|a| RustParam::Number(Qv::Val(*a))).collect()
    }
    pub(crate) fn from_params(v : &Vec<RustParam>) -> Option<RustNumArray>{
        let op  = v.iter().map(|p| p.to_num()).collect::<Option<Vec<f64>>>();
        Some(RustNumArray::new(op?))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct RustStrArray{
    b : Box<Vec<String>>,
}

impl RustStrArray{
    pub(crate) fn new(b : Vec<String>) -> RustStrArray{ RustStrArray{ b : Box::new(b) }}
    //pub(crate) fn as_ref(&self) -> &Vec<String>{ self.b.as_ref() }
    pub(crate) fn to_params(&self) -> Vec<RustParam>{
        self.b.iter().map(|a| RustParam::String(Qv::Val(RustString::new(a.to_string())))).collect()
    }
    pub(crate) fn from_params(v : &Vec<RustParam>) -> Option<RustStrArray>{
        let op : Option<Vec<String>> = v.iter().map(|p| p.to_string()).collect();
        Some(RustStrArray::new(op?))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct RustNum2Array{
    b : Box<Vec<Vec<f64>>>,
}

impl RustNum2Array{
    pub(crate) fn new(b : Vec<Vec<f64>>) -> RustNum2Array{ RustNum2Array{ b : Box::new(b) }}
    //pub(crate) fn as_ref(&self) -> &Vec<Vec<f64>>{ self.b.as_ref() }
    pub(crate) fn to_params(&self) -> Vec<RustParam>{
        self.b.iter().map(|a| RustParam::NumArray(Qv::Val(RustNumArray::new(a.clone())))).collect()
    }
    pub(crate) fn from_params(v : &Vec<RustParam>) -> Option<RustNum2Array>{
        let op : Option<Vec<Vec<f64>>> = v.iter().map(|p| p.to_num_array()).collect();
        Some(RustNum2Array::new(op?))
    }
}

