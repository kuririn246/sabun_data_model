
/// Iterator内部のバイト構造が分かれば、そのままCに構造体で返して、そのポインタをRust側に渡せば、
/// Iteratorがヒープ確保してるはずないと思うので、destroyなしで使える使いやすいCライブラリが出来るんじゃないかという気がするが
/// 無理矢理過ぎると思うので、Vecにコピーしてiterationするかなり保守的なやつを作った
#[repr(C)]
#[derive(Debug)]
pub struct CEnumerator<T>{
    vec : Vec<T>,
    counter : usize,
}

impl<T> CEnumerator<T>{
    pub fn new(vec : Vec<T>)-> CEnumerator<T>{
        CEnumerator{ vec, counter : 0 }
    }
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
