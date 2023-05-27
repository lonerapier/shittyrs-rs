#[derive(PartialEq, Debug)]
pub enum Errors {
    TooFewNumbers,
    TooFewParity,
    TooManyNumbers,
    InvalidMessage,
}

impl Errors {
    fn to_string(&self) -> &str {
        match *self {
            Self::TooFewNumbers => "Total number provided is less than one",
            Self::TooFewParity => "Total parity number provided is less than one",
            Self::TooManyNumbers => "Total number provided is greater than order of the field",
            Self::InvalidMessage => "Invalid message to encode",
        }
    }
}

impl std::fmt::Display for Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl std::error::Error for Errors {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
