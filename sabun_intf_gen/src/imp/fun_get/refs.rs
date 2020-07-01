use sabun_maker::structs::VarType;
use crate::imp::to_struct_temp_from_struct_desc::{push, with_old, with_var};

pub fn get_ref_fun_string(id : &str, snake_name : &str, is_old : bool, var_type : VarType, self_mod_name: &str, proxy_name : &str, result_type_name: &str) -> String{
    let mut s = String::new();
    push(&mut s, 0, &format!("pub fn ref_{}(&mut self) -> &{}{{\n", with_old(snake_name, is_old), with_var(result_type_name, var_type)));
    push(&mut s, 1,&format!("if let Some(v) = &self.{}{{\n", proxy_name));
    push(&mut s, 2,&format!("return v);\n"));
    push(&mut s, 1,&format!("}}\n"));
    push(&mut s, 1,&format!("let qv = {}::get_ref(self.ptr, \"{}\").unwrap();\n", self_mod_name, id));
    match &var_type {
        VarType::Normal => {
            push(&mut s, 1,&format!("let ans = qv.into_value().unwrap();\n"));
        },
        VarType::Undefiable => {
            push(&mut s, 1,&format!("let ans = UndefOr.from_qv(qv).unwrap();\n"));
        },
        VarType::Nullable => {
            push(&mut s, 1,&format!("let ans = NullOr.from_qv(qv).unwrap();\n"));
        },
        VarType::UndefNullable => {
            push(&mut s, 1,&format!("let ans = qv;\n"));
        },
    }
    push(&mut s, 1,&format!("self.{} = Some({}::new(ans));\n", proxy_name, result_type_name));
    push(&mut s, 1,&format!("return self.{}.as_ref().unwrap();\n", proxy_name));
    push(&mut s, 0,"}");
    s
}