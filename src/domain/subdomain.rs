use std::{fmt::Display, str::FromStr};

use super::DomainError;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SubDomain(String);

impl Display for SubDomain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl SubDomain {
    pub fn is_valid(subdomain: &str) -> bool {
        if subdomain.is_empty() {
            return false;
        }

        if subdomain.starts_with('.') || subdomain.ends_with('.') {
            return false;
        }

        if subdomain.starts_with('-') || subdomain.ends_with('-') {
            return false;
        }

        subdomain.chars().all(Self::is_valid_char)
    }

    fn is_valid_char(char: char) -> bool {
        matches!(char, 'a'..='z' | '0'..='9' | '.' | '-')
    }
}

impl FromStr for SubDomain {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if ! Self::is_valid(s) {
            return Err(Self::Err::InvalidSubdomain);
        }

        Ok(Self(s.to_string()))
    }
}
