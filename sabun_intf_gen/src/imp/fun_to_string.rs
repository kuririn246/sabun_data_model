use crate::imp::fun::Contents;
use crate::imp::fun::Fun;
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

    match &fun.result_type{
        Some(t) =>{ s.push_str(&format!(") -> {} {{", t)); }
        None =>{ s.push_str("){") }
    }

    result.push(StrAndTab::new(s, 0));


    match &fun.contents{
        Contents::Get(g) =>{

            result.push(StrAndTab::new(
                format!("{}::get_{}(unsafe{{ self.ptr.as_ref().unwrap() }}, \"{}\");", self_mod_name, &g.type_name_small, &fun.name), 1));
            result.push(StrAndTab::new("}".to_string(), 0));
        },
        Contents::Set(g) =>{
            result.push(StrAndTab::new(
                format!("{}::set_{}(unsafe{{ self.ptr.as_mut_ref().unwrap() }}, \"{}\", {});", self_mod_name, &g.type_name_small, &g.param_name, &g.param_name), 1));
            result.push(StrAndTab::new("}".to_string(), 0));
        },
    }

    result
}