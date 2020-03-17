use std::rc::Rc;
use indexmap::IndexMap;

#[derive(Clone, Debug, PartialEq)]
pub enum JVal{
    Null(Span),
    Bool(bool, Span),
    String(String, Span),
    //Int(i64, Span),
    Double(f64, Span),
    Array(Vec<JVal>, Span),
    Map(IndexMap<String, JVal>, Span)
}

impl JVal{
    pub fn as_object(&self) -> Option<&IndexMap<String, JVal>>{
        return match self {
            JVal::Map(map, _span) => { Some(map) }
            _ => { None }
        }
    }

    pub fn as_array(&self) -> Option<&Vec<JVal>>{
        return match self{
            JVal::Array(a, _) => { Some(a) },
            _ =>{ None }
        }
    }

    pub fn as_str(&self) -> Option<&str>{
        return match self {
            JVal::String(s, _span) => { Some(s) }
            _ => { None }
        }
    }

    pub fn as_num(&self) -> Option<f64>{
        return match self{
            //JVal::Int(i, _) =>{ Some(*i as f64) },
            JVal::Double(d, _) =>{ Some(*d) },
            _ =>{ None }
        }
    }

    pub fn as_bool(&self) -> Option<bool>{
        return match self{
            JVal::Bool(b, _) =>{ Some(*b) }
            _ =>{ None }
        }
    }

    pub fn is_null(&self) -> bool{
        return match self{
            JVal::Null(_) => true,
            _ => false,
        }
    }

    pub fn span(&self) -> &Span{
        return match self{
            JVal::Null(s) => s,
            JVal::Bool(_,s) => s,
            JVal::String(_, s) => s,
            //JVal::Int(_, s) => s,
            JVal::Double(_, s) => s,
            JVal::Array(_, s) => s,
            JVal::Map(_, s) => s,
        }
    }

    pub fn original(&self) -> &str{
        self.span().slice()
    }

    ///くっそ重いので正常系で実行しないように注意
    pub fn line_col(&self) -> String{
        self.span().line_col_str()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Span{
    pub start : usize,
    pub end : usize,
    pub text : Rc<String>,
}

impl Span{
    pub fn line_col(&self) -> (usize, usize) {
        let input = self.text.as_str();
        let mut pos = self.start;
        if pos > input.len() {
            panic!("position out of bounds");
        }

        // Position's pos is always a UTF-8 border.
        let slice = unsafe { std::str::from_utf8_unchecked(&input.as_bytes()[..pos]) };
        let mut chars = slice.chars().peekable();

        let mut line_col = (1, 1);

        while pos != 0 {
            match chars.next() {
                Some('\r') => {
                    if let Some(&'\n') = chars.peek() {
                        chars.next();

                        if pos == 1 {
                            pos -= 1;
                        } else {
                            pos -= 2;
                        }

                        line_col = (line_col.0 + 1, 1);
                    } else {
                        pos -= 1;
                        line_col = (line_col.0, line_col.1 + 1);
                    }
                }
                Some('\n') => {
                    pos -= 1;
                    line_col = (line_col.0 + 1, 1);
                }
                Some(c) => {
                    pos -= c.len_utf8();
                    line_col = (line_col.0, line_col.1 + 1);
                }
                None => unreachable!()
            }
        }

        line_col
    }

    ///くっそ重いので正常系で実行しないように注意
    pub fn line_col_str(&self) -> String{
        let (line, col) = self.line_col();
        format!("({}, {})", line, col)
    }

    pub fn slice(&self) -> &str{
        &self.text.as_str()[self.start..self.end]
    }
}

//impl Display for Span{
//    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//        let (line, col) = self.line_col();
//        write!(f, "({}, {})", line, col)
//    }
//
//}