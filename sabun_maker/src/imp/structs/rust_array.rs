use crate::imp::structs::qv::Qv;
use crate::imp::structs::rust_param::RustParam;
use crate::imp::structs::array_type::ArrayType;

#[derive(Debug, PartialEq, Clone)]
pub struct RustArray{
    array : Box<Qv<Vec<RustParam>>>,
}

impl RustArray{
    pub(crate) fn new(qv : Qv<Vec<RustParam>>) -> RustArray{
        RustArray{ array : Box::new(qv) }
    }

    pub(crate) fn from_int_array(qv : &Qv<RustIntArray>) -> RustArray{
        RustArray::new(qv.map(|a| a.to_params()))
    }

    pub(crate) fn from_float_array(qv : &Qv<RustFloatArray>) -> RustArray{
        RustArray::new(qv.map(|a| a.to_params()))
    }
    // pub(crate) fn from_str_array(qv : &Qv<RustStrArray>) -> RustArray{
    //     RustArray::new(qv.map(|a| a.to_params()))
    // }
    // pub(crate) fn from_num2_array(qv : &Qv<RustNum2Array>) -> RustArray{
    //     RustArray::new(qv.map(|a| a.to_params()))
    // }

    pub(crate) fn qv(&self) -> &Qv<Vec<RustParam>>{ self.array.as_ref() }

    pub(crate) fn to_float_array(&self) -> Option<Qv<RustFloatArray>>{
        self.qv().opt_map(|a| RustFloatArray::from_params(a))
    }

    pub(crate) fn to_int_array(&self) -> Option<Qv<RustIntArray>>{
        self.qv().opt_map(|a| RustIntArray::from_params(a))
    }

    // pub(crate) fn to_str_array(&self) -> Option<Qv<RustStrArray>>{
    //     self.qv().opt_map(|a| RustStrArray::from_params(a))
    // }
    //
    // pub(crate) fn to_num2_array(&self) -> Option<Qv<RustNum2Array>>{
    //     self.qv().opt_map(|a| RustNum2Array::from_params(a))
    // }

    pub(crate) fn to_param(&self, at : &ArrayType) -> Option<RustParam>{
        Some(match at{
            ArrayType::Float =>{ RustParam::FloatArray(self.to_float_array()?) },
            ArrayType::Int =>{ RustParam::IntArray(self.to_int_array()?) },
            //ArrayType::String =>{ RustParam::StrArray(self.to_str_array()?)}
            //ArrayType::Num2 =>{ RustParam::Num2Array(self.to_num2_array()?)}
        })
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct RustFloatArray{
    b : Box<Vec<f64>>,
}

impl RustFloatArray{
    pub(crate) fn new(b : Vec<f64>) -> RustFloatArray{ RustFloatArray{ b : Box::new(b) }}
    //pub(crate) fn as_ref(&self) -> &Vec<f64>{ self.b.as_ref() }
    pub(crate) fn to_params(&self) -> Vec<RustParam>{
        self.b.iter().map(|a| RustParam::Float(Qv::Val(*a))).collect()
    }
    pub(crate) fn from_params(v : &Vec<RustParam>) -> Option<RustFloatArray>{
        let op  = v.iter().map(|p| p.to_float()).collect::<Option<Vec<f64>>>();
        Some(RustFloatArray::new(op?))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct RustIntArray{
    b : Box<Vec<i64>>,
}

impl RustIntArray{
    pub(crate) fn new(b : Vec<i64>) -> RustIntArray{ RustIntArray{ b : Box::new(b) }}
    //pub(crate) fn as_ref(&self) -> &Vec<i64>{ self.b.as_ref() }
    pub(crate) fn to_params(&self) -> Vec<RustParam>{
        self.b.iter().map(|a| RustParam::Int(Qv::Val(*a))).collect()
    }
    pub(crate) fn from_params(v : &Vec<RustParam>) -> Option<RustIntArray>{
        let op  = v.iter().map(|p| p.to_int()).collect::<Option<Vec<i64>>>();
        Some(RustIntArray::new(op?))
    }
}


// #[derive(Debug, PartialEq, Clone)]
// pub struct RustStrArray{
//     b : Box<Vec<String>>,
// }
//
// impl RustStrArray{
//     pub(crate) fn new(b : Vec<String>) -> RustStrArray{ RustStrArray{ b : Box::new(b) }}
//     //pub(crate) fn as_ref(&self) -> &Vec<String>{ self.b.as_ref() }
//     pub(crate) fn to_params(&self) -> Vec<RustParam>{
//         self.b.iter().map(|a| RustParam::String(Qv::Val(RustString::new(a.to_string())))).collect()
//     }
//     pub(crate) fn from_params(v : &Vec<RustParam>) -> Option<RustStrArray>{
//         let op : Option<Vec<String>> = v.iter().map(|p| p.to_string()).collect();
//         Some(RustStrArray::new(op?))
//     }
// }
//
// #[derive(Debug, PartialEq, Clone)]
// pub struct RustNum2Array{
//     b : Box<Vec<Vec<f64>>>,
// }
//
// impl RustNum2Array{
//     pub(crate) fn new(b : Vec<Vec<f64>>) -> RustNum2Array{ RustNum2Array{ b : Box::new(b) }}
//     //pub(crate) fn as_ref(&self) -> &Vec<Vec<f64>>{ self.b.as_ref() }
//     pub(crate) fn to_params(&self) -> Vec<RustParam>{
//         self.b.iter().map(|a| RustParam::NumArray(Qv::Val(RustNumArray::new(a.clone())))).collect()
//     }
//     pub(crate) fn from_params(v : &Vec<RustParam>) -> Option<RustNum2Array>{
//         let op : Option<Vec<Vec<f64>>> = v.iter().map(|p| p.to_num_array()).collect();
//         Some(RustNum2Array::new(op?))
//     }
// }
//
