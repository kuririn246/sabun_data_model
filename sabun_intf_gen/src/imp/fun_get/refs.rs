use sabun_maker::structs::VarType;
use crate::imp::to_struct_temp_from_struct_desc::{push, with_old, with_var};

fn option_type_str(var : VarType) -> String{
    match var{
        VarType::Nullable => "NullOr".to_string(),
        VarType::Undefiable => "UndefOr".to_string(),
        VarType::UndefNullable => "Qv".to_string(),
        VarType::Normal => "".to_string(),
    }
}

pub fn get_ref_fun_string(col_name : &str, snake_name : &str, is_old : bool, var_type : VarType, self_mod_name: &str, proxy_name : &str, result_type_name: &str) -> String{
    let mut s = String::new();
    push(&mut s, 0, &format!("pub fn ref_{}(&mut self) -> &{}{{\n", with_old(snake_name, is_old), with_var(result_type_name, var_type)));
    push(&mut s, 1,&format!("if let Some(v) = self.{}{{\n", proxy_name));
    push(&mut s, 2,&format!("return unsafe{{ v.as_ref().unwrap() }};\n"));
    push(&mut s, 1,&format!("}}\n"));
    push(&mut s, 1,&format!("let qv = {}::get_ref(self.ptr, \"{}\").unwrap();\n", self_mod_name, col_name));
    if var_type != VarType::Normal {
        push(&mut s, 1, &format!("\
    let ref_id = match qv{{
        Qv::None =>{{ return {}::Null; }},\n", option_type_str(var_type)));
        push(&mut s, 2, &format!("\
        Qv::Undefined =>{{ return {}::Undefined; }},\n", option_type_str(var_type)));
        push(&mut s, 2, &format!("Qv::Val(id) =>id,
    }};"));
    } else{
        push(&mut s, 2, &format!("\
        let ref_id = if let Qv::Val(v) = qv{{ v }} else {{ unreachable!() }};\n"));
    }
    push(&mut s, 1,&format!("let root = unsafe{{ self.root.as_ref().unwrap() }};\n"));
    push(&mut s, 1,&format!("let ref_ptr : *const {} = root.{}().from_id(ref_id);\n", result_type_name, col_name));
    push(&mut s, 1,&format!("self.{} = Some(ref_ptr);\n", proxy_name));
    push(&mut s, 1,&format!("let pp = self.{}.as_ref().unwrap();\n", proxy_name));
    push(&mut s, 1,&format!("unsafe{{ (*pp).as_ref().unwrap() }}\n"));
    push(&mut s, 0,"}");
    s
}