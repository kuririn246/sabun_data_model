#[repr(u32)] #[derive(Debug, PartialEq, Clone, Copy)]
pub enum ParamType{
    Bool, Int, Float, String, IntArray, FloatArray,
}
impl ParamType{
    pub fn nickname(&self) -> &str{
        match self{
            ParamType::Bool => "bool",
            ParamType::Int => "int",
            ParamType::Float => "float",
            ParamType::String => "str",
            ParamType::IntArray => "int_array",
            ParamType::FloatArray => "float_array",
        }
    }
    pub fn typename(&self) -> &str{
        match self{
            ParamType::Bool => "bool",
            ParamType::Int => "i64",
            ParamType::Float => "f64",
            ParamType::String => "String",
            ParamType::IntArray => "Vec<i64>",
            ParamType::FloatArray => "Vec<f64>",
        }
    }
}