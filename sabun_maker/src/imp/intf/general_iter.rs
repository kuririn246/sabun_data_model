pub struct GeneralIter<T,U>{
    len : usize,
    counter : usize,
    intf : T,
    getter : fn(&T,usize) -> U,
}
impl<T,U> GeneralIter<T,U>{
    pub fn new(len : usize, intf : T, getter : fn(&T,usize) -> U) -> GeneralIter<T,U>{
        GeneralIter{ len, counter : 0, intf, getter }
    }
}

impl<T,U> Iterator for GeneralIter<T,U>{
    type Item = U;

    fn next(&mut self) -> Option<Self::Item> {
        if self.counter < self.len{
            let counter = self.counter;
            self.counter += 1;
            Some((self.getter)(&self.intf, counter))
        } else{
            None
        }
    }
}