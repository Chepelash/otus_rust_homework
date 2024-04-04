use std::{fmt::Display, str::FromStr};


pub enum Response {
    Ok { result: Option<String> },
    Error { reason: String },
}
impl Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ok { result } => {
                if result.is_some() {
                    writeln!(f, "Ok::{}", result.as_ref().unwrap())
                } else {
                    writeln!(f, "Ok")
                }
            }
            Self::Error { reason } => {
                writeln!(f, "Error::{}", reason)
            }
        }
    }
}
impl FromStr for Response {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut collection = s.split("::");
        let result = collection.next().unwrap_or_default();
        let options = collection.next();
        match result {
            "Ok" => Ok(Self::Ok {
                result: options.map(|s| s.to_string()),
            }),
            "Error" => Ok(Self::Error {
                reason: options.unwrap_or_default().to_string(),
            }),
            _ => Err(()),
        }
    }
}
