use crate::{HashM, HashS};
use crate::imp::structs::root_value::RootValue;
use crate::imp::structs::rust_param::RustParam;
use crate::imp::structs::util::set_sabun::{SetSabunError, verify_set_sabun};

#[derive(Debug, PartialEq)]
pub struct RootObject{
    ///listのobjectの場合、defaultはlist側にあるが、ここには初期値が入る。
    default : Box<HashM<String, RootValue>>,
    ///変更されたものを記録
    ///listの変更はMutListが直接上書きされるので、sabunには入らない。よってparamだけ記録される
    sabun : Box<HashM<String, RustParam>>,

    ///oldに設定されたメンバは、_Oldを付けなければプログラムから使用できず、
    ///ConstTableである場合、jsonで Refできない
    old : Box<HashS<String>>,
}

impl RootObject{
    pub(crate) fn new(default : HashM<String, RootValue>, sabun : HashM<String, RustParam>, old : HashS<String>) -> RootObject{
        RootObject{ default: Box::new(default), sabun : Box::new(sabun), old : Box::new(old) }
    }
    pub fn default(&self) -> &HashM<String, RootValue>{ self.default.as_ref() }
    pub(crate) fn default_mut(&mut self) -> &mut HashM<String, RootValue>{ self.default.as_mut() }
    pub(crate) fn deconstruct(self) -> (HashM<String, RootValue>, HashM<String, RustParam>, HashS<String>){ (*self.default, *self.sabun, *self.old) }
    pub fn sabun(&self) -> &HashM<String, RustParam>{ self.sabun.as_ref() }
    pub(crate) fn old(&self) -> &HashS<String>{ self.old.as_ref() }
    pub(crate) fn set_sabun(&mut self, name : String, param : RustParam) -> Result<Option<RustParam>, SetSabunError> {
        let (p, vt) = if let Some(RootValue::Param(p, vt)) = self.default().get(&name) { (p, vt) } else {
            return Err(SetSabunError::ParamNotFound);
        };
        verify_set_sabun(p, vt, &param)?;
        Ok(self.sabun.insert(name, param))
    }
}

