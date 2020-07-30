use std::fmt::{Display, Formatter};
use std::cell::RefCell;

#[derive(Debug, PartialEq, Clone)]
pub struct RustString{
    str : RefCell<String>,
}

impl RustString{
    pub(crate) fn new(s : String) -> RustString{ RustString{ str : RefCell::new(s) }}
    pub(crate) fn str(&self) -> &str{ self.str.as_ref().as_ref() }
    pub(crate) fn write(&mut self, s : String){
        self.str.replace(s);
    }
}

impl Display for RustString{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.str().fmt(f)
    }
}



