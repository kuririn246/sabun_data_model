use bit_vec::BitVec;
use crate::kihon_enum::KihonFromTag;

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

    pub fn read_tag(&mut self) -> KihonFromTag{
        let mut count = 0;
        loop{
            let b = self.read_bit();
            if b{ break; }
            count += 1;
        }
        match count{
            0 => return KihonFromTag::Bit(self.read_bit()),
            1 => return KihonFromTag::Byte,
            2 => return KihonFromTag::Str16(self.read_bits(4) as u8),
            3 =>{
                let bytes = self.read_bits(3);
                if bytes == 0{ return KihonFromTag::Float; }
                else{ return KihonFromTag::Int((bytes + 1) as u8); }
            },
            4 => return KihonFromTag::Str256,
            5 => return KihonFromTag::Double,
            6 =>{
                let bytes = self.read_bits(4);
                return KihonFromTag::Decimal((bytes + 1) as u8);
            },
            7 =>{
                let bytes = self.read_bits(3);
                return KihonFromTag::BigStr((bytes + 1) as u8);
            },
            _ =>{ panic!("Tag's zeroes must be within 7") }
        }
    }

    pub fn rest_of_the_vec(self) -> Vec<u8>{
        let mut index = self.index / 8;
        if self.index % 8 != 0{ index += 1; }
        Vec::from(&self.vec.to_bytes()[index..])
    }
}