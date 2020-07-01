use sabun_maker::structs::VarType;
use crate::imp::to_struct_temp_from_struct_desc::{push, with_old, with_var};

pub fn get_fun_string(name : &str, is_old : bool, var_type : VarType, self_mod_name : &str, self_type_name : &str,
                      value_nickname: &str, proxy_name : &str, value_type_name : &str) -> String{
    let mut s = String::new();
    push(&mut s, 0, &format!("pub fn {}(&self) -> &{}{{\n", with_old(name, is_old), with_var(value_type_name, var_type)));
    push(&mut s, 1,&format!("if let Some(v) = &self.{}{{\n", proxy_name));
    push(&mut s, 2,&format!("return v;\n"));
    push(&mut s, 1,&format!("}}\n"));
    push(&mut s, 1,&format!("let qv = {}::get_{}(self.ptr, \"{}\").unwrap();\n", self_mod_name, value_nickname, name));
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
    push(&mut s, 1,&format!("let mp = unsafe {{ (self as *const {} as *mut {}).as_mut().unwrap() }};\n", self_type_name, self_type_name));
    push(&mut s, 1,&format!("mp.{} = Some(ans);\n", proxy_name));
    push(&mut s, 1,&format!("return self.{}.as_ref().unwrap();\n", proxy_name));
    push(&mut s, 0,"}");
    s
}

