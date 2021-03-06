pub type Result<T> = std::result::Result<T, SabunError>;


#[derive(Clone, Debug, PartialEq)]
pub struct SabunError{
    pub message : String,
}


impl From<json5_parser::MyError> for SabunError {
    fn from(e : json5_parser::MyError) -> Self {
        SabunError{ message : e.message }
    }
}

impl From<std::io::Error> for SabunError {
    fn from(s : std::io::Error) -> Self {
        SabunError{ message : s.to_string() }
    }
}


impl From<String> for SabunError {
    fn from(s : String) -> Self {
        SabunError{ message : s }
    }
}

impl From<&str> for SabunError {
    fn from(s : &str) -> Self {
        SabunError{ message : s.to_string() }
    }
}