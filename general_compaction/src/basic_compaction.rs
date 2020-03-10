use crate::kihon_enum::Kihon;
use crate::enc_dec::var_int;
use std::f64;

///f64が整数で表せるなら整数で、bitやbyteに出来るならそこまで削って表す。f32にしても情報が失われない場合f32にする。
/// 動的型言語でnumがf64に決まっている場合や、静的型言語でf64に戻すことがわかっている場合に縮める事ができる。
pub fn comp_double(num : f64) -> Kihon{
    //-0.0は整数としては扱えない。
    //0.0 == -0.0はtrueなので面倒な比較をする。
    if num.to_bits() == (-0.0f64).to_bits(){ return Kihon::Double(num); }

    let int = num.trunc();
    //整数ではない
    if int != num{ return Kihon::Double(num) }
    if int as i64 as f64 == int {
        //Intが8byteだとDoubleにくらべて全然小さくならないしむしろ1bit大きくなる・・・？？
        return comp_int(int as i64);
    } else if num as f32 as f64 == num {
        return Kihon::Float(num as f32);
    } else {
        return Kihon::Double(num);
    }
}

///f32が整数で表せるなら整数で、bitやbyteに出来るならそこまで削って表す。
pub fn comp_float(num : f32) -> Kihon{
    //-0.0は整数としては扱えない。
    //0.0 == -0.0はtrueなので面倒な比較をする。
    if num.to_bits() == (-0.0f32).to_bits(){ return Kihon::Float(num) }

    let int = num.trunc();
    //整数ではない
    if int != num{ return Kihon::Float(num) }
    //それ以上だと節約にならないので、i32までの数値ならという条件をつける。
    if int as i32 as f32 == int {
        return comp_int(int as i64);
    } else{
        return Kihon::Float(num);
    }
}

///bitやbyteに出来るならそこまで削って表す。
pub fn comp_int(int : i64) -> Kihon{
    if int == 0{ return Kihon::Bit(false); }
    if int == 1{ return Kihon::Bit(true); }
    if int as i8 as i64 == int{ return Kihon::Byte(int as i8); }
    return Kihon::Int(int);
}

///文字列を適切な形式で保存する。
/// 静的型言語等で文字列に戻すことが確定している場合で、数値を文字列で表現している場合には、
/// string_compactionのメソッドを使って文字列を数値に変換して保存して容量を節約することも出来る。
pub fn comp_str(s : String) -> Kihon{
    let len = s.len();
    if len < 16{ return Kihon::Str16(s); }
    if len < 256{ return Kihon::Str256(s); }
    return Kihon::BigStr(s);
}
