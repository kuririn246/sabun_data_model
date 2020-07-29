use sabun_maker::structs::{VarType, ParamType};

#[repr(C)] #[derive(Debug, PartialEq)]
pub struct ParamSource{
    name : String,
    var_type : VarType,
    param_type : ParamType,
    get : bool,
    set : bool,
    c_get : bool,
    c_set : bool,
}
impl ParamSource{
    pub fn new(name : String, var_type : VarType, param_type : ParamType,
               get : bool, set : bool, c_get : bool, c_set : bool) -> ParamSource{
        ParamSource{
            name, var_type, param_type,
            get, set, c_get, c_set
        }
    }

    pub fn name(&self) -> &str{
        &self.name
    }
    pub fn var_type(&self) -> VarType{
        self.var_type
    }
    pub fn param_type(&self) -> ParamType{
        self.param_type
    }
    pub fn get(&self) -> Option<&str>{
        unimplemented!()
    }
    pub fn set(&self) -> Option<&str>{
        unimplemented!()
    }
    pub fn c_get(&self) -> Option<&str>{
        unimplemented!()
    }
    pub fn c_set(&self) -> Option<&str>{
        unimplemented!()
    }
}
fn opt(s : &str) -> Option<&str>{
    if s.is_empty(){ None }
    else{ Some(s) }
}