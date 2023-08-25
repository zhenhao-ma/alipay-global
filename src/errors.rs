use std::fmt::format;

use serde::Serialize;

/// Custom Error for Alipay Response
/// Fail: Indicates that the API call fails.
/// Unknown: Indicates that the API call might be successful, in process, or failed. For more details, see Result process logic.
#[derive(Debug, Serialize)]
pub enum Error {
    Fail(String),
    Unknown(String),
}

impl From<ureq::Error> for Error {
    fn from(value: ureq::Error) -> Self {
        match value {
            ureq::Error::Status(x, res) => Self::Fail(format!(
                "Request Status code {}, {}",
                x.to_string(),
                res.into_string().unwrap()
            )),
            ureq::Error::Transport(t) => {
                Self::Fail(format!("Request transport error: {}", t.to_string()))
            }
        }
    }
}
impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::Unknown(format!("Std IO Error: {}", value.to_string()))
    }
}
