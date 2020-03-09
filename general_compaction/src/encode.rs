use crate::kihon_enum::Kihon;
use crate::tag_storage::TagStorage;

fn write_byte(i : i8, tag : &mut TagStorage, data : &mut Vec<u8>){
    tag.append(0b01, 2); data.push(i as u8);
}

pub fn encode(vec : &Vec<Kihon>) -> Vec<u8>{
    let mut data : Vec<u8> = vec![];
    let mut tag = TagStorage::new();
    for item in vec{
        match item{
            Kihon::Bit(b) =>{ tag.append(0b10 + *b as u64, 2) }
            Kihon::Byte(i) =>{
                write_byte(*i, &mut tag, &mut data);
            },
            Kihon::Str16(s) =>{
                tag.append(0b001, 3);
                if s.len() < 16 {
                    tag.append(s.len() as u64, 4);
                } else{
                    panic!("Small str's len must be within 15");
                }
                data.extend_from_slice(s.as_bytes());
            },
            Kihon::Int(i) =>{
                let ans = crate::var_int::encode(*i);
                let size = ans.len();
                if 8 < size { panic!("Int's size must be 2..=8"); }
                if size == 1{
                    //sizeが1の場合Intとしては保存できないので、勝手にByteに変える。
                    write_byte(ans[0] as i8, &mut tag, &mut data);
                } else {
                    tag.append(0b0001, 4);
                    tag.append((size - 1) as u64, 3);
                    data.extend_from_slice(&ans);
                }
            },
            Kihon::Float(f) =>{
                tag.append(0b0001, 4);
                tag.append(0, 3);
                data.extend_from_slice(&f.to_be_bytes());
            },
            Kihon::Str256(s) =>{
                //if s.len() <= 15 || 256 <= s.len(){
                  //  panic!("Str256's size must be 16..=255")
                //}
                tag.append(0b0000_1, 5);
                data.push(s.len() as u8);
                data.extend_from_slice(s.as_bytes());
            },
            Kihon::Double(d) =>{
                tag.append(0b0000_01, 6);
                data.extend_from_slice(&d.to_be_bytes());
            },
            Kihon::Decimal(i, dot) =>{
                let i = *i; let dot = *dot;
                let ans = crate::var_int::encode128(i);
                let size = ans.len();
                if size <= 0 || 17 <= size{ panic!("decimal's size must be 1..=16"); }

                tag.append(0b0000_001, 7);
                tag.append((size-1) as u64, 4);
                data.extend_from_slice(&ans);
                data.push(dot);
            }
            Kihon::BigStr(s)=>{
                tag.append(0b0000_0001, 8);
                let vec = crate::var_int::encode(s.len() as i64);
                if 8 < vec.len(){ panic!("BigStr is too large"); }
                tag.append((vec.len() - 1) as u64, 3);
                data.extend_from_slice(&vec);
                data.extend_from_slice(s.as_bytes());
            }
        }
    }
    let mut result : Vec<u8> = vec![];
    //最初にアイテム数が入る。
    let len = vec.len() as i64;
    let bytes = crate::var_int::encode(len);
    result.push(bytes.len() as u8);
    result.extend_from_slice(&bytes);
    result.append(&mut tag.to_vec());
    result.append(&mut data);
    return result;
}
