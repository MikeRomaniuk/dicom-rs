use quick_error::quick_error;

/// Type alias for a result from this crate.
pub type Result<T> = ::std::result::Result<T, Error>;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Io(err: std::io::Error) {
            from()
        }
        FromUtf8(err: std::string::FromUtf8Error) {
            from()
        }
        Generic(err: String) {
            from()
            from(err: &str) -> (err.to_string())
            display("{}", err)
        }
        NoPDUAvailable {

        }
        InvalidMaxPDU {

        }
        PDUTooLarge {

        }
    }
}
