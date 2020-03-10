pub mod b8s_and_b6s;
pub mod char_and_b6;


pub fn get_url_string(bytes : &[u8]) -> String{
    let b6s = self::b8s_and_b6s::to_b6s(bytes);
    self::char_and_b6::b6s_to_string(&b6s).unwrap()
}

pub fn get_bytes(url_string : &str) -> Option<Vec<u8>>{
    let b6s = self::char_and_b6::string_to_b6s(url_string)?;
    Some(self::b8s_and_b6s::to_b8s(&b6s))
}