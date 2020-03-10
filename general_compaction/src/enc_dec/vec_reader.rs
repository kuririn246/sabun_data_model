pub struct VecReader{
    pub vec : Vec<u8>,
    pub index : usize,
}

impl VecReader{
    pub fn new(vec : Vec<u8>) -> VecReader{
        VecReader{ vec, index : 0 }
    }

    pub fn read(&mut self) -> u8{
        let r = self.vec[self.index];
        self.index += 1;
        return r;
    }

    pub fn read_vec(&mut self, len : usize) -> Vec<u8>{
        let mut vec : Vec<u8> = vec![];
        for _ in 0..len{
            vec.push(self.read());
        }
        return vec;
    }

    pub fn read_string(&mut self, len : usize) -> String{
        String::from_utf8(self.read_vec(len)).unwrap()
    }
}

