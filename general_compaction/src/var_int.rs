use crate::signed_bytes::{signed_bytes, signed_bytes128};

pub fn encode(val : i64) -> Vec<u8>{
    let sign;
    let value;
    if val < 0{
        sign = false;
        //127..-128 のような感じなので、マイナスの方が一つ多い。そこを補正しつつ-1もかけることで、0-127に収める。
        value = ((val + 1) * -1) as u64;
    }  else{
        sign = true;
        value = val as u64;
    }
    let mut bytes = to_bytes(value);
    let signed_bytes = signed_bytes(val);
    if bytes.len() < signed_bytes{ bytes.push(0); }

    if sign == false{
        (*bytes.last_mut().unwrap()) |= 0b1000_0000; //負の場合左端を1に
    }
    return bytes;
}

pub fn decode(vec : Vec<u8>) -> i64{
    if vec.len() == 0{ return 0; }
    let mut vec = vec;
    let sign;
    let last_val = *vec.last().unwrap();
    if last_val & 0b1000_0000 != 0{
        sign = false;
        (*vec.last_mut().unwrap()) ^= 0b1000_0000; //左端を反転
    } else{
        sign = true;
    }
    let mut ans = from_bytes(&vec) as i64;
    if sign == false{
        ans = (ans * -1) - 1;
    }
    return ans;
}

pub fn to_bytes(val : u64) -> Vec<u8>{
    if val == 0{ return vec![0]; } //0は1byteと考える・・・本当は0バイトで表せると思うけれど。
    let mut val = val;
    let mut result : Vec<u8> = vec![];
    loop{
        if val == 0{ break; }
        result.push((val % 256) as u8);
        val = val / 256;
    }
    return result;
}

///一応汎用ライブラリのつもりだから、パフォーマンスのための最適化として128bit版と64bit版を用意する。
pub fn to_bytes128(val : u128) -> Vec<u8>{
    if val == 0{ return vec![0]; } //0は1byteと考える・・・本当は0バイトで表せると思うけれど。
    let mut val = val;
    let mut result : Vec<u8> = vec![];
    loop{
        if val == 0{ break; }
        result.push((val % 256) as u8);
        val = val / 256;
    }
    return result;
}

pub fn from_bytes(vec : &Vec<u8>) -> u64{
    let mut result : u64 = 0;
    for i in vec.iter().rev(){
        result *= 256;
        result += (*i) as u64;
    }
    return result;
}

pub fn from_bytes128(vec : &Vec<u8>) -> u128{
    let mut result : u128 = 0;
    for i in vec.iter().rev(){
        result *= 256;
        result += (*i) as u128;
    }
    return result;
}

pub fn encode128(val : i128) -> Vec<u8>{
    let sign;
    let uval;
    if val < 0{
        sign = false;
        uval = ((val +1) * -1) as u128;
    } else{
        sign = true;
        uval = val as u128;
    }
    let mut bytes : Vec<u8> = to_bytes128(uval);
    let signed_bytes = signed_bytes128(val);
    if bytes.len() < signed_bytes{ bytes.push(0); }

    if sign == false{
        (*bytes.last_mut().unwrap())  |= 0b1000_0000;
    }
    return bytes;
}

pub fn decode128(vec : Vec<u8>) -> i128{
    if vec.len() == 0{ return 0; }
    let mut vec = vec;
    let sign;
    let last_val = *vec.last().unwrap();
    if last_val & 0b1000_0000 != 0{
        sign = false;
        (*vec.last_mut().unwrap()) ^= 0b1000_0000; //左端を反転
    } else{
        sign = true;
    }
    let mut ans = from_bytes128(&vec) as i128;
    if sign == false{
        ans = (ans * -1) - 1;
    }
    return ans;
}