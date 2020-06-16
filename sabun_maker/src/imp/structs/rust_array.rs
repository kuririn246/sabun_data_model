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
    pub(crate) fn from_num_array(qv : &Qv<Vec<f64>>) -> RustArray{
        RustArray::new(qv.map(|a| a.iter().map(|f| RustParam::Number(Qv::Val(*f))).collect()))
    }
    pub(crate) fn from_str_array(qv : &Qv<Vec<String>>) -> RustArray{
        RustArray::new(qv.map(|a| a.iter().map(|f| RustParam::String(Qv::Val(RustString::new(f.to_string())))).collect()))
    }
    pub(crate) fn from_num2_array(qv : &Qv<Vec<Vec<f64>>>) -> RustArray{
        RustArray::new(qv.map(|a| a.iter().map(|f| RustParam::NumArray(Qv::Val(f.clone()))).collect()))
            //RustArray::from_num_array(&Qv::Val(f.clone())), ArrayType::Num)).collect()))
    }

    // pub(crate) fn null(at : ArrayType) -> RustArray{
    //     RustArray{ array : Box::new(RustArrayInternal{
    //         qv : Qv::Null, at
    //     })}
    // }
    // pub(crate) fn undefined(at : ArrayType) -> RustArray{
    //     RustArray{ array : Box::new(RustArrayInternal{
    //         qv : Qv::Undefined, at
    //     })}
    // }
    pub(crate) fn qv(&self) -> &Qv<Vec<RustParam>>{ self.array.as_ref() }
    //pub(crate) fn array_type(&self) -> ArrayType{ self.array.at.clone() }

    pub(crate) fn to_num_array(&self) -> Result<Qv<Vec<f64>>,()>{
        let v = match self.array.as_ref() {
            Qv::Null => { return Ok(Qv::Null) },
            Qv::Undefined => { return Ok(Qv::Undefined) },
            Qv::Val(v) => v,
        };

        let mut result : Vec<f64> = Vec::with_capacity(v.len());
        for p in v {
            if let RustParam::Number(Qv::Val(s)) = p {
                result.push(*s);
            } else {
                return Err(());
            }
        }
        return Ok(Qv::Val(result));
    }

    pub(crate) fn to_str_array(&self) -> Result<Qv<Vec<String>>,()>{
        let v = match self.array.as_ref() {
            Qv::Null => { return Ok(Qv::Null) },
            Qv::Undefined => { return Ok(Qv::Undefined) },
            Qv::Val(v) => v,
        };

        let mut result : Vec<String> = Vec::with_capacity(v.len());
        for p in v {
            if let RustParam::String(Qv::Val(s)) = p {
                result.push(s.str().to_string());
            } else {
                return Err(());
            }
        }
        return Ok(Qv::Val(result));
    }

    pub(crate) fn to_num2_array(&self) -> Result<Qv<Vec<Vec<f64>>>,()>{
        let v = match self.array.as_ref() {
            Qv::Null => { return Ok(Qv::Null) },
            Qv::Undefined => { return Ok(Qv::Undefined) },
            Qv::Val(v) => v,
        };

        let mut result : Vec<Vec<f64>> = Vec::with_capacity(v.len());
        for p in v {
            if let RustParam::NumArray(Qv::Val(a)) = p {
                result.push(a.clone());
            } else{
                return Err(());
            }
        }
        return Ok(Qv::Val(result));
    }

    pub(crate) fn to_param(&self, at : &ArrayType) -> Result<RustParam, ()>{
        Ok(match at{
            ArrayType::Num =>{ RustParam::NumArray(self.to_num_array()?) },
            ArrayType::String =>{ RustParam::StrArray(self.to_str_array()?)}
            ArrayType::Num2 =>{ RustParam::Num2Array(self.to_num2_array()?)}
        })
    }
}