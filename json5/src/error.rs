use pest;

use crate::de::Rule;
//use std::error::Error;

pub type Result<T> = std::result::Result<T, MyError>;

#[derive(Clone, Debug, PartialEq)]
pub struct MyError{
    pub message : String,
    pub source : String,
    pub start : Option<usize>,
    pub end : Option<usize>,
}

impl MyError{
    pub fn new(s : String, start : usize) -> MyError{
        MyError{ message : s.to_string(), source : s, start : Some(start), end : None }
    }
}


impl From<pest::error::Error<Rule>> for MyError {
    fn from(err: pest::error::Error<Rule>) -> Self {
        match err.location{
            pest::error::InputLocation::Pos(start) =>{
                MyError{ message : err.to_string(), source : err.to_string(), start : Some(start), end : None }
            },
            pest::error::InputLocation::Span((start, end)) =>{
                MyError{ message : err.to_string(), source : err.to_string(), start : Some(start),  end : Some(end) }
            }
        }
    }
}
//
// imp ser::Error for Error {
//     fn custom<T: Display>(msg: T) -> Self {
//         Error::Message(msg.to_string(), None)
//     }
// }
//
// imp de::Error for Error {
//     fn custom<T: Display>(msg: T) -> Self {
//         Error::Message(msg.to_string(), None)
//     }
// }
//
// imp Display for Error {
//     fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
//         formatter.write_str(self.description());
//         //formatter.write_str(std::error::Error::description(self))
//     }
// }
//
// imp std::error::Error for Error {
//     fn description(&self) -> &str {
//         match *self {
//             Error::Message(ref msg, _) => msg,
//         }
//     }
// }
