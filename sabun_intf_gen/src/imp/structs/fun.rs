use sabun_maker::structs::VarType;

pub struct Fun{
    pub(crate) name : String,
    pub(crate) args : Vec<Arg>,
    pub(crate) result_type : Option<String>,
    //pub(crate) is_mut : bool,
    pub(crate) contents : Contents,
}
fn to_opt_string(s :&str) -> Option<String>{ if s.is_empty(){ None } else{ Some(s.to_string()) }}

impl Fun{
    //pub(crate) fn new_c(name : String, args : Vec<Arg>, result_type : Option<String>, is_mut : bool, contents : Contents) -> Fun{
      //  Fun{ name, args, result_type, is_mut, contents }
    //}
    pub(crate) fn new(name : &str, args : Vec<Arg>, result_type : &str, contents : Contents) -> Fun{
        Fun{ name : name.to_string(), args, result_type : to_opt_string(result_type), contents }
    }
}

pub struct Arg{
    pub(crate) name : String,
    pub(crate) arg_type : String,
}
impl Arg{
    pub fn new(name : &str, arg_type : &str) -> Arg{
        Arg{ name : name.to_string(), arg_type : arg_type.to_string() }
    }
}

pub enum Contents{
    Get(GetC),
    Set(SetC),
}

pub struct GetC{
    pub(crate) type_name_small: String,
    pub(crate) vt : VarType,
    pub(crate) proxy_name : String,
}
impl GetC{
    pub fn new(type_name_small: &str, vt : VarType, proxy_name : &str) -> GetC{ GetC{ type_name_small: type_name_small.to_string(), vt, proxy_name : proxy_name.to_string() }}
}

pub struct SetC{
    pub(crate) type_name_small : String,
    pub(crate) param_name : String,
    pub(crate) vt : VarType,
    pub(crate) proxy_name : String,
}
impl SetC{
    pub fn new(type_name_small : &str, param_name : &str, vt : VarType, proxy_name : &str) -> SetC{
        SetC{ type_name_small : type_name_small.to_string(), param_name : param_name.to_string(), vt, proxy_name : proxy_name.to_string() }}
}