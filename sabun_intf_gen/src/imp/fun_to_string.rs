use crate::imp::fun::Fun;
use crate::imp::fun::Contents;
use crate::imp::str_and_tab::StrAndTab;

pub(crate) fn fun_to_string(fun : &Fun, self_mod_name : &str) -> Vec<StrAndTab>{
    let mut result = Vec::new();
    let mut s = String::new();
    let self_ = if fun.is_mut{ format!("&mut self") } else{ format!("&self") };
    s.push_str(&format!("pub fn {}({}, ", &fun.name, self_));
    for arg in &fun.args{
        s.push_str(&format!("{} : {}, ", &arg.name, &arg.arg_type));
    }
    s.pop(); s.pop();

    s.push_str(&format!(") -> {} {{", &fun.result_type));
    result.push(StrAndTab::new(s, 0));


    match &fun.contents{
        Contents::Get(g) =>{
            let mut s = String::new();
            s.push_str(&format!("{}::get_{}(self, \"{}\"", self_mod_name, &g.result_type_name_small, &fun.name));
            result.push(StrAndTab::new(s, 1));
            result.push(StrAndTab::new("}".to_string(), 0));
        }
    }

    result
}