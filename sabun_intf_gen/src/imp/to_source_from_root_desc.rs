use crate::imp::structs::struct_desc::{StructDesc, RefItem, ParamItem};
use crate::imp::structs::struct_temp::{StructTemp};
use sabun_maker::structs::VarType;
use crate::imp::util::to_type_name::{to_item_type_name, to_snake_name};
use crate::imp::fun_get::param::get_fun_string;
use crate::imp::fun_get::col::get_col_fun_string;
use crate::imp::fun_set::param_fun_set::fun_set;
use crate::imp::fun_get::refs::get_ref_fun_string;

pub fn to_source_from_root_desc(d : &StructDesc) -> String{

    let mut param_funs = params_to_funs(&d.params, &d.item_mod_name,  d.is_mut);
    let mut col_funs = cols_to_funs(d);
    param_funs.append(&mut col_funs);
    let (funs, proxies) = separate(param_funs.clone());


}
