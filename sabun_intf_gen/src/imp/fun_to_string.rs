use crate::imp::fun::Contents;
use crate::imp::fun::Fun;
use crate::imp::str_and_tab::StrAndTab;
use sabun_maker::structs::VarType;

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
                format!("let qv = {}::get_{}(unsafe{{ self.ptr.as_ref().unwrap() }}, \"{}\").unwrap();", self_mod_name, &g.type_name_small, &fun.name), 1));
            match &g.vt {
                VarType::Normal => {
                    result.push(StrAndTab::new(
                            format!("qv.into_value().unwrap()"), 1));
                },
                VarType::Undefiable =>{
                    result.push(StrAndTab::new(
                        format!("UndefOr.from_qv(qv).unwrap()"), 1));
                },
                VarType::Nullable =>{
                    result.push(StrAndTab::new(
                        format!("NullOr.from_qv(qv).unwrap()"), 1));
                },
                VarType::UndefNullable =>{
                    result.push(StrAndTab::new(
                        format!("qv"), 1));
                },
            }

        },
        Contents::Set(g) =>{
            let param = if g.vt == VarType::Normal{ format!("Qv::Val({})", &g.param_name)} else{ format!("{}.into_qv()", &g.param_name)};
            result.push(StrAndTab::new(
                format!("{}::set_{}(unsafe{{ self.ptr.as_mut().unwrap() }}, \"{}\", {});", self_mod_name, &g.type_name_small, &g.param_name, &param), 1));

        },
    }
    result.push(StrAndTab::new("}".to_string(), 0));

    result
}

