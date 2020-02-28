
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