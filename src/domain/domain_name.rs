use std::{fmt::Display, str::FromStr, ops::Deref};

use super::DomainError;


#[derive(Debug)]
pub struct DomainName(String);

impl DomainName {

    pub fn get_name(&self) -> String {
        self.0.clone()
    }

    fn is_valid_char(char: char) -> bool {
        matches!(char, 'a'..='z' | '0'..='9' | '-')
    }

   pub fn is_valid(name: &str) -> bool {
        if name.is_empty() {
            return false;
        }

        if name.starts_with('-') | name.ends_with('-') {
            return false;
        }

        name.chars().all(Self::is_valid_char)
    }
}

impl Display for DomainName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for DomainName {
    //type Err = DomainError;
    type Err = DomainError;
    
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let value = value.to_lowercase();

        if ! Self::is_valid(value.as_str()) {
            return Err(Self::Err::InvalidName);
        }

        Ok(Self(value))
    }
}

