#[derive(Debug, PartialEq)]
pub enum Kihon{
    Null,
    Bit(bool),
    Bool(bool),
    Byte(i8),
    Str16(String),
    Int(i64),
    Float(f32),
    Str256(String),
    Double(f64),
    ///u8はドットの位置を表す。0はドットがないことを表す。
    Decimal(i128, u8),
    BigStr(String),
    Undefined(u8),
}

pub struct Decimal{
    pub int : i128,
    pub dot : u8,
}

impl Decimal{
    pub fn to_f64(&self) -> f64 {
        crate::enc_dec::decimal_lib::to_f64(self.int, self.dot)
    }
}

impl Kihon{
    ///bit,byte,int,float,double can be converted to f64
    pub fn as_f64(&self) -> Option<f64>{
        use Kihon::*;
        match self{
            Bit(b) => Some(*b as f64),
            Byte(b) => Some(*b as f64),
            Int(i) => Some(*i as f64),
            Float(f) => Some(*f as f64),
            Double(f) => Some(*f),

            _ => None,
        }
    }

    ///bit, byte, int
    pub fn as_i64(&self) -> Option<i64>{
        use Kihon::*;
        match self {
            Bit(b) => Some(*b as i64),
            Byte(b) => Some(*b as i64),
            Int(i) => Some(*i as i64),
            _ => None,
        }
    }

    ///str16, str256, BigStr
    pub fn as_string(&self) -> Option<String>{
        use Kihon::*;
        match self{
            Str16(s) => Some(s.to_string()),
            Str256(s) => Some(s.to_string()),
            BigStr(s) => Some(s.to_string()),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool>{
        match self{
            Kihon::Bool(b) => Some(*b),
            _ => None,
        }
    }

    pub fn as_null(&self) -> Option<()>{
        match self{
            Kihon::Null => Some(()),
            _ => None,
        }
    }

    pub fn as_undefined(&self) -> Option<u8>{
        match self{
            Kihon::Undefined(u) => Some(*u),
            _ => None,
        }
    }

    pub fn as_decimal(&self) -> Option<(i128, u8)>{
        match self{
            Kihon::Decimal(i, u) => Some((*i, *u)),
            _ => None,
        }
    }



    pub fn to_string(&self) -> String{
        crate::string_compaction::to_string(self)
    }
}