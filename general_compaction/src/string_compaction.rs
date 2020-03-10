use crate::kihon_enum::Kihon;
use regex::{Regex, Match};
use crate::enc_dec::encode::encode;
use crate::enc_dec::decode::decode;
use crate::enc_dec::decimal_lib;
use lazy_static::lazy_static;

pub fn to_kihons(strs : Vec<String>) -> Vec<Kihon>{
    strs.iter().map(|s| to_enum(s)).collect()
}

///文字列が数値であれば数値として保存することで容量を節約する。
pub fn compress_strings(strs : Vec<String>) -> Vec<u8>{
    let vec : Vec<Kihon> = to_kihons(strs);
    let encoded = encode(&vec);
    return encoded;
}

pub fn decompress(bytes : Vec<u8>) -> Vec<String>{
    let kihons = decode(bytes);
    to_strings(&kihons)
}

pub fn to_strings(kihons : &Vec<Kihon>) -> Vec<String>{
    kihons.iter().map(|k| to_string(k)).collect()
}

pub fn to_string(k : &Kihon) -> String{
    fn s(t : &str) -> String{ t.to_string() }

    return match k{
        Kihon::Null => s("null"),
        Kihon::Bit(b) => if *b{ s("1") } else{ s("0") }
        Kihon::Bool(b) => if *b{ s("true")} else{ s("false") }
        Kihon::Byte(b) => b.to_string(),
        Kihon::Str16(s) => s.to_string(),
        Kihon::Int(i) => i.to_string(),
        Kihon::Float(f) => f.to_string(),
        Kihon::Str256(s) => s.to_string(),
        Kihon::Double(d) => d.to_string(),
        Kihon::Decimal(num, dot) => decimal_lib::to_string(*num, *dot),
        Kihon::BigStr(s) => s.to_string(),
        Kihon::Undefined(u) => format!("undefined{}", u),
    }
}

fn to_enum(s : &str) -> Kihon{
    if s == "0"{ return Kihon::Bit(false) }
    if s == "1"{ return Kihon::Bit(true) }
    if s == "true"{ return Kihon::Bool(true) }
    if s == "false"{ return Kihon::Bool(false) }

    if let Some(number) = get_number(s){
        if number.number == 0 && number.minus {
            //-0はこの仕組では表現できない
            return get_str(s);
        } else if let Some(dot) = number.dot{
            return Kihon::Decimal(number.number, dot);
        } else{
            if number.number == number.number as i64 as i128{
                //i64で損失なく表現可能である
                let number = number.number as i64;
                if number == number as i8 as i64{
                    //i8で損失なく表現可能である
                    return Kihon::Byte(number as i8);
                } else {
                    return Kihon::Int(number as i64);
                }
            } else{
                return Kihon::Decimal(number.number, 0);
            }
        }
    } else { return get_str(s); }
}

fn get_str(s : &str) -> Kihon{
    if s.len() < 16{
        return Kihon::Str16(s.to_string());
    } else if s.len() < 256{
        return Kihon::Str256(s.to_string());
    } else{
        return Kihon::BigStr(s.to_string());
    }
}

struct NumResult{
    pub minus : bool,
    pub dot : Option<u8>,
    pub number : i128,
}

fn rex(s : &str) -> Option<regex::Captures>{
    lazy_static! {
        static ref RE: Regex = regex::Regex::new(r#"^(-?)(([0-9]+)(.?)([0-9]*))$"#).unwrap();
    }
    return RE.captures(s);
}

///dot位置が255桁まで 整数がi128までのdecimalとして表現可能かを調べる。leading zerosは認めないか、0.のときの一個のみ。
/// 最後が.である数値も認めない。
fn get_number(s : &str) -> Option<NumResult>{

    if let Some(caps) = rex(s){
        let minus = (&caps[1]).len() != 0;
        fn get_dot_info(m : Option<Match>, len : usize) -> Result<Option<u8>, ()>{
            if let Some(m) = m{
                if m.as_str().len() != 0 {
                    let pos = len - m.start() - 1;
                    if pos < 256 {
                        return Ok(Some(pos as u8));
                    } else{
                        return Err(());
                    }
                }
            }
            return Ok(None);
        }

        let dot = get_dot_info(caps.get(4), s.len()).ok()?;

        let num_str = format!("{}{}{}",&caps[1], &caps[3], &caps[5]);

        let number = num_str.parse::<i128>().ok()?;
        let result = NumResult{ minus, dot, number };

        let s = &caps[2];
        if s.ends_with("."){
            return None;
        }
        if s.starts_with("0"){
            if s.starts_with("0."){
                return Some(result);
            } else{
                return None;
            }
        } else{
            return Some(result);
        }
    } else{ return None; }
}