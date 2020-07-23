

/// &参照を露出してしまうと、それが生きている間にwriteしたら当然UB(undefined behavior)になる。
/// 参照を見せないために、RustではStringのCopyを行う
/// 対してCからは、ポインタを介してアクセスする分にはUBにならないのでコピーしない(そもそもコピーしても破棄するのが大変・・・
#[repr(C)] #[derive(Debug, Clone, Copy)]
pub struct RustStrPtr{
    s : *const String,
}
impl RustStrPtr{
    pub fn new(s : *const String) -> RustStrPtr{ RustStrPtr{ s } }
    pub fn to_string(&self) -> String{
        let s = unsafe{&*self.s };
        s.to_string()
    }
}

#[no_mangle] #[allow(non_snake_case)]
pub extern "C" fn RustStrPtr_len(p : RustStrPtr) -> u64{
    let s = unsafe{&*p.s};
    s.len() as u64
}
#[no_mangle] #[allow(non_snake_case)]
pub extern "C" fn RustStrPtr_ptr(p : RustStrPtr) -> *const u8{
    let s = unsafe{&*p.s};
    s.as_ptr()
}