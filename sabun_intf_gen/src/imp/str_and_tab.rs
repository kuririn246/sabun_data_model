pub struct StrAndTab{
    pub(crate) s : String,
    pub(crate) tab : usize,
}

impl StrAndTab{
    pub fn new(s : String, tab : usize) -> StrAndTab{ StrAndTab{ s, tab }}
}

pub fn str_and_tabs_to_string(ss : &[StrAndTab]) -> String{
    let mut r : String = String::new();
    for s in ss{
        for _ in 0..s.tab{
            r.push('\t');
        }
        r.push_str(&s.s);
        r.push('\n');
    }
    r
}