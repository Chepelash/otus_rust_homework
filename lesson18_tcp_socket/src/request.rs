use std::str::FromStr;

use crate::command::Command;

#[derive(Debug, Default)]
pub enum RequestType {
    #[default]
    Get,
    Post,
    Del,
    Put,
}

impl FromStr for RequestType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().trim() {
            "get" => Ok(Self::Get),
            "post" => Ok(Self::Post),
            "del" => Ok(Self::Del),
            "put" => Ok(Self::Put),
            _ => Err(()),
        }
    }
}
#[derive(Debug)]
pub struct Request {
    pub req_type: RequestType,
    pub command: Command,
}
impl Request {
    pub fn new(req_type: RequestType, command: Command) -> Self {
        Self { req_type, command }
    }
}
impl Default for Request {
    fn default() -> Self {
        Self {
            req_type: RequestType::Get,
            command: Command::ShowMain,
        }
    }
}
