use std::{fmt::Display, str::FromStr};

use super::DomainError;

#[derive(Debug, PartialEq, Eq, Clone)]
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
    type Err = DomainError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let value = value.to_lowercase();

        if !Self::is_valid(value.as_str()) {
            return Err(Self::Err::InvalidName);
        }

        Ok(Self(value))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn valid_domain_name_from_str() {
        let names = vec![
            ("example", true),
            (".", false),
            ("", false),
            ("example.", false),
            ("example.net", false),
            ("university", true),
            ("example123", true),
            ("example ", false),
            ("still-valid", true),
            ("-not-valid", false),
            ("not-valid-either-", false),
            ("not_valid", false),
        ];

        for (name, expected) in names {
            let domain_name = DomainName::from_str(name);

            assert_eq!(domain_name.is_ok(), expected);
        }
    }

    #[test]
    fn domain_matches_expected_str() {
        let names = vec![
            ("EXAMPLE", "example"),
            ("EXamPLE", "example"),
            ("EXAM123ple", "exam123ple"),
        ];

        for (name, expected) in names {
            let domain_name = DomainName::from_str(name);

            assert!(domain_name.is_ok());

            if let Ok(domain_name) = domain_name {
                assert_eq!(domain_name.to_string(), expected);
            }
        }
    }
}
