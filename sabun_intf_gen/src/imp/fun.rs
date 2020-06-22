use sabun_maker::structs::VarType;

pub struct Impl{
    pub(crate) funs : Vec<Fun>,
    pub(crate) self_mod_name : String,
    pub(crate) struct_name : String,
    pub(crate) ptr_type : String,
}

pub struct Fun{
    pub(crate) name : String,
    pub(crate) args : Vec<Arg>,
    pub(crate) result_type : Option<String>,
    pub(crate) is_mut : bool,
    pub(crate) contents : Contents,
}
fn to_opt_string(s :&str) -> Option<String>{ if s.is_empty(){ None } else{ Some(s.to_string()) }}

impl Fun{
    pub(crate) fn new_c(name : String, args : Vec<Arg>, result_type : Option<String>, is_mut : bool, contents : Contents) -> Fun{
        Fun{ name, args, result_type, is_mut, contents }
    }
    pub(crate) fn new(name : &str, args : Vec<Arg>, result_type : &str, is_mut : bool, contents : Contents) -> Fun{
        Fun{ name : name.to_string(), args, result_type : to_opt_string(result_type), is_mut, contents }
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
}
impl GetC{
    pub fn new(type_name_small: &str, vt : VarType) -> GetC{ GetC{ type_name_small: type_name_small.to_string(), vt }}
}

pub struct SetC{
    pub(crate) type_name_small : String,
    pub(crate) param_name : String,
    pub(crate) vt : VarType,
}
impl SetC{
    pub fn new(type_name_small : &str, param_name : &str, vt : VarType) -> SetC{
        SetC{ type_name_small : type_name_small.to_string(), param_name : param_name.to_string(), vt }}
}