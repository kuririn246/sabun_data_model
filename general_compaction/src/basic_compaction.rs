use crate::kihon_enum::Kihon;
use crate::enc_dec::var_int;
use std::f64;

///f64が整数で表せるなら整数で、bitやbyteに出来るならそこまで削って表す。
pub fn comp_num(num : f64) -> Kihon{
    //-0.0は扱えない
    if num.to_bits() == (-0.0f64).to_bits(){ Kihon::Double(num) }

    let int = num.trunc();
    //整数ではない
    if int != num{ Kihon::Double(num) }
    if int as i64 as f64 == int {
        return from_int(int as i64);
    } else{
        return Kihon::Double(num);
    }
}

///bitやbyteに出来るならそこまで削って表す。
pub fn comp_int(int : i64) -> Kihon{
    if int == 0{ return Kihon::Bit(false); }
    if int == 1{ return Kihon::Bit(true); }
    if int as i8 as i64 == int{ return Kihon::Byte(int as i8); }
    return Kihon::Int(int);
}

