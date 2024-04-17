use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct Er<'a> {
    msg: &'a str,
}

impl <'a> Er<'a> {
    pub fn new(msg: &'a str) -> Self {
        Self { msg }
    }
}

impl Error for Er<'_> { }

impl Display for Er<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}