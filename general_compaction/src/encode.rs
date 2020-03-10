use crate::kihon_enum::Kihon;
use crate::tag_storage::TagStorage;


pub fn encode(vec : &Vec<Kihon>) -> Vec<u8>{
    let mut data : Vec<u8> = vec![];
    let mut tag = TagStorage::new();
    for item in vec{
        match item{
            Kihon::Null =>{ tag.append(0b0100_1, 5) }
            Kihon::Bit(b) =>{ tag.append(0b10 + *b as u64, 2) }
            Kihon::Bool(b) =>{ tag.append(0b0100_010 + *b as u64, 7) }
            Kihon::Byte(i) =>{
                tag.append(0b011, 3);
                data.push(*i as u8);
            },
            Kihon::Str16(s) =>{
                tag.append(0b0101, 4);
                if s.len() < 16 {
                    tag.append(s.len() as u64, 4);
                } else{
                    panic!("Small str's len must be within 15");
                }
                data.extend_from_slice(s.as_bytes());
            },
            Kihon::Int(i) => {
                let ans = crate::var_int::encode(*i);
                let size = ans.len();
                if 8 < size { panic!("Int's size must be 1..=8"); }
                tag.append(0b0011, 4);
                tag.append((size - 1) as u64, 3);
                data.extend_from_slice(&ans);
            },
            Kihon::Float(f) =>{
                tag.append(0b0010_0001, 8);
                data.extend_from_slice(&f.to_be_bytes());
            },
            Kihon::Str256(s) =>{
                tag.append(0b0010_1, 5);
                data.push(s.len() as u8);
                data.extend_from_slice(s.as_bytes());
            },
            Kihon::Double(d) =>{
                tag.append(0b0010_01, 6);
                data.extend_from_slice(&d.to_be_bytes());
            },
            Kihon::Decimal(i, dot) =>{
                let i = *i; let dot = *dot;
                let ans = crate::var_int::encode128(i);
                let size = ans.len();
                if size <= 0 || 17 <= size{ panic!("decimal's size must be 1..=16"); }

                tag.append(0b0100_001, 7);
                tag.append((size-1) as u64, 4);
                data.extend_from_slice(&ans);
                data.push(dot);
            }
            Kihon::BigStr(s)=>{
                tag.append(0b0010_001, 7);
                let vec = crate::var_int::encode(s.len() as i64);
                if 8 < vec.len(){ panic!("BigStr is too large"); }
                tag.append((vec.len() - 1) as u64, 3);
                data.extend_from_slice(&vec);
                data.extend_from_slice(s.as_bytes());
            },
            Kihon::Undefined(i)=>{
                if 7 < *i{ panic!("Undefined must be 0..=7") }
                tag.append(0b0001, 4);
                tag.append(0, *i as usize);
                tag.append(1, 1);
            },

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
