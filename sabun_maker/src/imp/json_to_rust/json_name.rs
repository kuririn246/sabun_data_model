use regex::Regex;
use regex::Captures;
use lazy_static::lazy_static;
use crate::structs::value_type::ValueType;

pub enum SystemNames{
    Include,
    Renamed,
    ID,
    Ref,
    Obsolete,
}

pub enum NameType{
    Name(String, ValueType),
    SystemName(SystemNames)
}

pub fn json_name(s : &str) -> Option<NameType>{
    fn some(sn: SystemNames) -> Option<NameType> {
        return Some(NameType::SystemName(sn));
    }
    use SystemNames::*;

    match s {
        "Include" => return some(Include),
        "Renamed" => return some(Renamed),
        "ID" => return some(ID),
        "Ref" => return some(Ref),
        "Obsolete" => return some(Obsolete),
        _ => {},
    }

    let (vt, name) = value_type_and_name(s)?;
    return Some(NameType::Name(name, vt))
}


// ///[@a-z_][a-zA-Z0-9_]*
// pub fn is_valid_name(s : &str) -> bool{
//     lazy_static! {
//         static ref RE : Regex = Regex::new(r"^[@a-z_][a-zA-Z0-9_]*$").unwrap();
//     }
//     RE.is_match(s)
// }


///[@a-z_][a-zA-Z0-9_]*
pub fn analyze_name(s : &str) -> Option<Captures>{
    lazy_static! {
        static ref RE : Regex = Regex::new(r"^([@a-z_][a-zA-Z0-9_]*)([!?]*)$").unwrap();
    }
    RE.captures(s)
}

pub fn value_type_and_name(s : &str) -> Option<(ValueType, String)>{
    if let Some(cap) = analyze_name(s){
        let name = cap[1].to_string();
        let suffix = cap[2].to_string();
        let value_type = if suffix == "!?" || suffix == "?!"{
            ValueType::UndefNullable
        } else if suffix == "!"{
            ValueType::Undefinable
        } else if suffix == "?"{
            ValueType::Nullable
        } else if suffix == ""{
            ValueType::Normal
        } else{
            return None;
        };
        return Some((value_type, name))
    } else{
        return None;
    }
}
//
/////[a-z_][a-zA-Z0-9]*\??
//pub fn is_possible_name(s : &str) -> Option<(bool, &str)>{
//    let (b,s) = is_nullable(s);
//    if is_valid_name(s){
//        return Some((b,s))
//    } else{
//        return None;
//    }
//}