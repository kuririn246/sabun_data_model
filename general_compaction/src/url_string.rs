pub fn get_url_string(bytes : &[u8]) -> String{
    let b6s = crate::b8s_and_b6s::to_b6s(bytes);
    crate::char_and_b6::b6s_to_string(&b6s).unwrap()
}

pub fn get_bytes(url_string : &str) -> Option<Vec<u8>>{
    let b6s = crate::char_and_b6::string_to_b6s(url_string)?;
    Some(crate::b8s_and_b6s::to_b8s(&b6s))
}