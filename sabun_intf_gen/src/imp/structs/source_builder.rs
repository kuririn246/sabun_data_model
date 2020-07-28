use crate::imp::structs::str_and_tab::StrAndTab;

pub struct SourceBuilder{
    pub(crate) s : String,
}
impl SourceBuilder{
    pub(crate) fn new() -> SourceBuilder{ SourceBuilder{ s : String::new() }}
    pub(crate) fn push(&mut self, tabs : usize, s : &str){
        for line in s.split('\n'){
            self.s.push_str(&tab_line(tabs, line))
        }
    }
    pub(crate) fn to_string(&self) -> String{ self.s.to_string()
    }
}

pub struct StrAndTab{
    pub(crate) s : String,
    pub(crate) tab : usize,
}

impl StrAndTab{
    pub(crate) fn new(s : String, tab : usize) -> StrAndTab{ StrAndTab{ s, tab }}
}

fn tab_line(tabs : usize, s : &str) -> String{
    let mut r : String = String::new();
    for _ in 0..tabs{
        r.push('\t');

    }
    r.push_str(s);
    r.push('\n');
    r
}