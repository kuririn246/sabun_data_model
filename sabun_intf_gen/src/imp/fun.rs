
pub struct Impl{
    pub(crate) funs : Vec<Fun>,
    pub(crate) self_mod_name : String,
}

pub struct Fun{
    pub(crate) name : String,
    pub(crate) args : Vec<Arg>,
    pub(crate) result_type : String,
    pub(crate) is_mut : bool,
    pub(crate) contents : Contents,
}

impl Fun{
    pub(crate) fn new_c(name : String, args : Vec<Arg>, result_type : String, is_mut : bool, contents : Contents) -> Fun{
        Fun{ name, args, result_type, is_mut, contents }
    }
    pub(crate) fn new(name : &str, args : Vec<Arg>, result_type : &str, is_mut : bool, contents : Contents) -> Fun{
        Fun{ name : name.to_string(), args, result_type : result_type.to_string(), is_mut, contents }
    }
}

pub struct Arg{
    pub(crate) name : String,
    pub(crate) arg_type : String,
}

pub enum Contents{
    Get(GetC)
}

pub struct GetC{
    pub(crate) result_type_name_small : String,
}

impl GetC{
    pub fn new(result_type_name_small : &str) -> GetC{ GetC{ result_type_name_small : result_type_name_small.to_string() }}
}