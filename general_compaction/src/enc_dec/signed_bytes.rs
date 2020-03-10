

///1byteなら-128..=127まで、2byteなら-32768..32767というように数が収まるsignedのバイト数を計算する
pub fn signed_bytes(val : i64) -> usize{
    let mut val = if val < 0{ ((val + 1) * -1) as u64 } else{ val as u64 };
    let mut size = 1;
    loop{
        if val < 128{ return size; }
        val = val / 256;
        size += 1;
    }
}

pub fn signed_bytes128(val : i128) -> usize{
    let mut val = if val < 0{ ((val + 1) * -1) as u128 } else{ val as u128 };
    let mut size = 1;
    loop{
        if val < 128{ return size; }
        val = val / 256;
        size += 1;
    }
}