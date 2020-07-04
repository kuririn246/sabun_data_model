use crate::imp::to_struct_temp_from_struct_desc::{push, with_old};

pub fn get_col_fun_string(id : &str, snake_name : &str, is_old : bool, self_mod_name : &str, value_nickname: &str,
                          proxy_name : &str, col_type : &str) -> String{
    let mut s = String::new();

    push(&mut s, 0, &format!("pub fn {}(&mut self) -> &mut {}{{\n", with_old(snake_name, is_old), col_type));
    push(&mut s, 1,&format!("if self.{}.is_some() {{\n", proxy_name));
    push(&mut s, 2,&format!("return self.{}.as_mut().unwrap();\n", proxy_name));
    push(&mut s, 1,&format!("}}\n"));
    push(&mut s, 1,&format!("let ans = {}::get_{}(self.ptr, \"{}\").unwrap();\n", self_mod_name, value_nickname, id));

    push(&mut s, 1,&format!("self.{} = Some({}::new(ans, self.root()));\n", proxy_name, col_type));
    push(&mut s, 1,&format!("return self.{}.as_mut().unwrap();\n", proxy_name));
    push(&mut s, 0,"}");
    s
}