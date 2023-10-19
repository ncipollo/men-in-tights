#[derive(Debug, PartialEq)]
pub struct RobinhoodError {
    message: String,
}

impl RobinhoodError {
    pub fn new(message: &str) -> Self {
        Self {
            message: format!("😱 {message}"),
        }
    }
}

impl From<reqwest::Error> for RobinhoodError {
    fn from(value: reqwest::Error) -> Self {
        Self {
            message: format!("😱 https error: {value}")
        }
    }
}