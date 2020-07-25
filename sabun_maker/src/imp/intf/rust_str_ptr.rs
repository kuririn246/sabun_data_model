use crate::imp::structs::qv::Qv;
use std::ptr::null;

/// &参照を露出してしまうと、それが生きている間にwriteしたら当然UB(undefined behavior)になる。
/// 参照を見せないために、RustではStringのCopyを行う
/// 対してCからは、ポインタを介してアクセスする分にはUBにならないのでコピーしない(そもそもコピーしても破棄するのが大変・・・
///
/// MutListItemが削除されるとポインタは不正になる
/// このポインタがdefaultを指していると、Stringがsetされた場合、defaultの情報を指し続けることになり、不整合になる。
/// String自体は生きていても、stringのcapacity変更に伴う再構築が起きれば、ptrから取れる*const u8は無効になる
///
/// まあ一般的に、書き込んだ後はこのポインタは無効になると考えるべきだろう。
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

#[repr(C)] #[derive(Debug, Clone, Copy)]
pub struct QvStrPtr{
    s : *const Qv<String>,
}
impl QvStrPtr{
    pub fn new(s : *const Qv<String>)-> QvStrPtr{ QvStrPtr{ s } }
    pub fn value(&self) -> Qv<String>{ unsafe{(&*self.s).into_qv() } }
}
#[no_mangle] #[allow(non_snake_case)]
pub extern "C" fn QvStrPtr_NullOr(p : QvStrPtr) -> RustStrPtr{
    let p = unsafe{ &*p.s };
    match p{
        Qv::Val(s) => RustStrPtr::new(s),
        Qv::Null => RustStrPtr::new(null()),
        _ => unreachable!(),
    }
}
#[no_mangle] #[allow(non_snake_case)]
pub extern "C" fn QvStrPtr_UndefOr(p : QvStrPtr) -> RustStrPtr{
    let p = unsafe{ &*p.s };
    match p{
        Qv::Val(s) => RustStrPtr::new(s),
        Qv::Undefined => RustStrPtr::new(null()),
        _ => unreachable!(),
    }
}
#[no_mangle] #[allow(non_snake_case)]
pub extern "C" fn QvStrPtr_QvValue(p : QvStrPtr) -> RustStrPtr{
    let p = unsafe{ &*p.s };
    match p{
        Qv::Val(s) => RustStrPtr::new(s),
        _ => unreachable!(),
    }
}
#[no_mangle] #[allow(non_snake_case)]
pub extern "C" fn QvStrPtr_IsNull(p : QvStrPtr) -> i8{
    let p = unsafe{ &*p.s };
    match p{
        Qv::Null => 1,
        _ => 0,
    }
}
#[no_mangle] #[allow(non_snake_case)]
pub extern "C" fn QvStrPtr_IsUndefined(p : QvStrPtr) -> i8{
    let p = unsafe{ &*p.s };
    match p{
        Qv::Undefined => 1,
        _ => 0,
    }
}