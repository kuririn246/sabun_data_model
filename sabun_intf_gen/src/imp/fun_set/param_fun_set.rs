use crate::imp::to_struct_temp_from_struct_desc::{push, with_old, with_var};
use sabun_maker::structs::VarType;

pub fn fun_set(id : &str, snake_name : &str, is_old : bool, arg_type : &str, var_type : VarType, item_mod_name: &str, p: &str) -> String {
    let mut s = String::new();
    push(&mut s, 0, &format!("pub fn set_{}(&mut self, {} : {}){{\n", with_old(snake_name, is_old), id, with_var(arg_type, var_type)));
    push(&mut s, 1, &format!("self.{} = Some({});\n", &p, id));
    let param = if var_type == VarType::Normal { format!("Qv::Val({})", id) } else { format!("{}.into_qv()", id) };

    push(&mut s, 1, &format!("{}::set_bool(self.ptr, \"{}\", {});\n", item_mod_name, id, &param));
    push(&mut s, 0, "}");
    s
}