use regex::Regex;

pub enum SystemNames{
    Include,
    Rename,
    ID,
    RefID,
    RefIDs
}

pub enum NameType{
    Normal,
    SystemName(SystemNames)
}

pub fn json_name(s : &str) -> Result<NameType,String>{
    fn ok(sn : SystemNames) -> Result<NameType, String>{
        return Ok(NameType::SystemName(sn));
    }
    use SystemNames::*;

    match s{
        "Include" => return ok(Include),
        "Rename" => return ok(Rename),
        "ID" => return ok(ID),
        "RefID" => return ok(RefID),
        "RefIDs" => return ok(RefIDs),
        _=>{},
    }
    let re = Regex::new(r"[a-z][a-zA-Z0-9]*").unwrap();
    if re.is_match(s){
        Ok(NameType::Normal)
    } else{
        Err(format!("{} is not a valid name [a-z][a-zA-Z0-9]*",s))
    }
}
