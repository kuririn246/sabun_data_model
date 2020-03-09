use json5_parser::Span;
pub type Result<T> = std::result::Result<T, SabunError>;
use std::result::Result as StdResult;

#[derive(Clone, Debug, PartialEq)]
pub struct SabunError{
    pub message : String,
}


impl From<json5_parser::MyError> for SabunError {
    fn from(e : json5_parser::MyError) -> Self {
        SabunError{ message : e.message }
    }
}

//impl From<&str> for SabunError {
//    fn from(s : &str) -> Self {
//        SabunError{ message : s.to_string() }
//    }
//}


impl From<String> for SabunError {
    fn from(s : String) -> Self {
        SabunError{ message : s }
    }
}