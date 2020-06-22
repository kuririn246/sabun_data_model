pub struct StrAndTab{
    pub(crate) s : String,
    pub(crate) tab : usize,
}

impl StrAndTab{
    pub fn new(s : String, tab : usize) -> StrAndTab{ StrAndTab{ s, tab }}
}