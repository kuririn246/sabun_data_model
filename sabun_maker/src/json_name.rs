use regex::Regex;
use lazy_static::lazy_static;

pub enum SystemNames{
    Include,
    Rename,
    ID,
    RefID,
    RefIDs
}

pub enum NameType{
    Normal,
    Nullable(String),
    SystemName(SystemNames)
}

pub fn json_name(s : &str) -> Option<NameType>{
    fn some(sn: SystemNames) -> Option<NameType> {
        return Some(NameType::SystemName(sn));
    }
    use SystemNames::*;

    match s {
        "Include" => return some(Include),
        "Rename" => return some(Rename),
        "ID" => return some(ID),
        "RefID" => return some(RefID),
        "RefIDs" => return some(RefIDs),
        _ => {},
    }

    let (is_nullable, s) = is_possible_name(s)?;

    if is_nullable {
        Some(NameType::Nullable(s.to_string()))
    } else {
        Some(NameType::Normal)
    }
}

///[a-z_][a-zA-Z0-9]*
pub fn is_valid_name(s : &str) -> bool{
    lazy_static! {
        static ref RE : Regex = Regex::new(r"[a-z_][a-zA-Z0-9]*").unwrap();
    }
    RE.is_match(s)
}


pub fn is_nullable(s : &str) -> (bool, &str){
    if s.ends_with("?"){
        (true, &s[0..s.len()-1])
    } else{
        (false, s)
    }
}

///[a-z_][a-zA-Z0-9]*\??
pub fn is_possible_name(s : &str) -> Option<(bool, &str)>{
    let (b,s) = is_nullable(s);
    if is_valid_name(s){
        return Some((b,s))
    } else{
        return None;
    }
}