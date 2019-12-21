pub struct ConvertFromJsonError{
    pub message : String,
}

//impl From<io::Error> for MyError {
//    fn from(e: io::Error) -> MyError {
//        MyError::FileWriteError
//    }
//}