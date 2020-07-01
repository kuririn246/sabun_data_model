use crate::imp::to_struct_temp_from_struct_desc::{push, with_old};

pub fn get_col_fun_string(name : &str, is_old : bool, item_mod_name : &str, value_nickname: &str, proxy_name : &str, col_type : &str) -> String{
    let mut s = String::new();
    push(&mut s, 0, &format!("pub fn {}(&mut self) -> {}{{\n", with_old(name, is_old), col_type));
    push(&mut s, 1,&format!("if let Some(v) = &self.{}{{\n", proxy_name));
    push(&mut s, 2,&format!("return v.clone();\n"));
    push(&mut s, 1,&format!("}}\n"));
    push(&mut s, 1,&format!("let ans = {}::get_{}(self.ptr, \"{}\").unwrap();\n", item_mod_name, value_nickname, name));

    push(&mut s, 1,&format!("self.{} = Some({}::new(ans));\n", proxy_name, col_type));
    push(&mut s, 1,&format!("return self.{}.clone().unwrap();\n", proxy_name));
    push(&mut s, 0,"}");
    s
}