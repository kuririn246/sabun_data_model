use crate::kihon_enum::{Kihon};
use super::vec_reader::VecReader;
use super::tag_reader::TagReader;
use crate::enc_dec::kihon_from_tag::KihonFromTag;

pub fn decode(vec : Vec<u8>) -> Vec<Kihon>{
    let mut vec = VecReader::new(vec);
    let size = vec.read() as usize;
    let count = super::var_int::decode(vec.read_vec(size)) as usize;
    let mut reader = TagReader::new(&vec.vec[vec.index..]);
    let mut tags : Vec<KihonFromTag> = vec![];
    for _ in 0..count{
        tags.push(reader.read_tag());
    }

    let mut data = VecReader::new(reader.rest_of_the_vec());

    let mut result : Vec<Kihon> = vec![];
    for tag in tags{
        let kihon = match tag{
            KihonFromTag::Null => Kihon::Null,
            KihonFromTag::Bit(b) => Kihon::Bit(b),
            KihonFromTag::Bool(b) => Kihon::Bool(b),
            KihonFromTag::Byte => Kihon::Byte(data.read() as i8),
            KihonFromTag::Str16(l) =>{
                Kihon::Str16(data.read_string(l as usize ))
            },
            KihonFromTag::Int(size) => {
                let size = size as usize;
                let bytes = data.read_vec(size);
                Kihon::Int(super::var_int::decode(bytes))
            },
            KihonFromTag::Float =>{
                let v = data.read_vec(4);
                let f : f32 = f32::from_be_bytes([v[0],v[1],v[2],v[3]]);
                Kihon::Float(f)
            },
            KihonFromTag::Str256 =>{
                let size = data.read() as usize;
                Kihon::Str256(data.read_string(size))
            },
            KihonFromTag::Double =>{
                let v = data.read_vec(8);
                let d : f64 = f64::from_be_bytes([v[0],v[1],v[2],v[3],v[4],v[5],v[6],v[7]]);
                Kihon::Double(d)
            },
            KihonFromTag::Decimal(size) =>{
                let size = size as usize;
                let vec = data.read_vec(size);
                let v = super::var_int::decode128(vec);
                let dot = data.read();
                Kihon::Decimal(crate::kihon_enum::Decimal::new(v, dot))
            },
            KihonFromTag::BigStr(size) =>{
                let v = data.read_vec(size as usize);
                let len = super::var_int::decode(v);
                let s = data.read_string(len as usize);
                Kihon::BigStr(s)
            },
            KihonFromTag::Undefined(i) => Kihon::Undefined(i),
        };
        result.push(kihon);
    }

    return result;
}
//
//fn to_u128(vec : &[u8]) -> u128{
//    let mut r : u128 = 0;
//    for i in vec{
//        r *= 256;
//        r += *i as u128;
//    }
//    return r;
//}