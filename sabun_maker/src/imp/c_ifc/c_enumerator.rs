#[repr(C)]
pub struct CEnumerator<T>{
    vec : Vec<T>,
    counter : usize,
}

impl<T> CEnumerator<T>{
    pub fn new(vec : Vec<T>)-> CEnumerator<T>{ CEnumerator{ vec, counter : 0 }}
    pub fn current(&self) -> *mut T{
        if self.counter < self.vec.len(){
            let ptr = self.vec.get(self.counter).unwrap();
            return ptr as *const T as *mut T;
        } else{
            return 0 as *mut T;
        }
    }
    pub fn move_next(&mut self) -> bool{
        if self.counter < self.vec.len() - 1{
            self.counter += 1;
            return true;
        } else{
            return false;
        }
    }
}