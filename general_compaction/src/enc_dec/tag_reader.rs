use bit_vec::BitVec;
use crate::enc_dec::kihon_from_tag::KihonFromTag;


pub struct TagReader{
    pub vec : BitVec,
    pub index : usize,
}

impl TagReader{
    pub fn new(bytes : &[u8]) -> TagReader{
        TagReader{ vec : BitVec::from_bytes(bytes), index : 0 }
    }

    pub fn read_bit(&mut self) -> bool{
        let r = self.vec[self.index];
        self.index += 1;
        return r;
    }

    pub fn read_bits(&mut self, size : usize) -> u64{
        let mut r : u64 = 0;
        for _ in 0..size{
            r *= 2;
            r += self.read_bit() as u64;
        }
        return r;
    }

    pub fn read_next_1(&mut self) -> usize{
        let mut count = 0;
        loop{
            let b = self.read_bit();
            if b{ return count; }
            count += 1;
        }
    }

    pub fn read_tag(&mut self) -> KihonFromTag{
        let count = self.read_next_1();
        match count{
            0 => return KihonFromTag::Bit(self.read_bit()),
            1 =>{
                let count = self.read_next_1();
                match count{
                    0 => return KihonFromTag::Byte,
                    1 => return KihonFromTag::Str16(self.read_bits(4) as u8),
                    2 => return KihonFromTag::Null,
                    3 => return KihonFromTag::Bool(self.read_bit()),
                    4 => {
                        let bytes = self.read_bits(4);
                        return KihonFromTag::Decimal((bytes + 1) as u8);
                    },
                    _ =>{ panic!("undefined tag 0100_000")}
                }
            }
            2 => {
                let count = self.read_next_1();
                match count{
                    0 => {
                        let bytes = self.read_bits(3);
                        return KihonFromTag::Int((bytes + 1) as u8);
                    },
                    1 => return KihonFromTag::Str256,
                    2  => return KihonFromTag::Double,
                    3 => {
                        let bytes = self.read_bits(3);
                        return KihonFromTag::BigStr((bytes + 1) as u8);
                    },
                    4 => return KihonFromTag::Float,
                    _ => panic!("undefined tag 0010_0000")
                }
            },
            3 =>{
                let count = self.read_next_1();
                return KihonFromTag::Undefined(count as u8);
            },
            _ =>{ panic!("Tag's zeroes must be within 3") }
        }
    }

    pub fn rest_of_the_vec(self) -> Vec<u8>{
        let mut index = self.index / 8;
        if self.index % 8 != 0{ index += 1; }
        Vec::from(&self.vec.to_bytes()[index..])
    }
}