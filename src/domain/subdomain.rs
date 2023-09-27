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

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let value = value.to_lowercase();

        if !Self::is_valid(&value) {
            return Err(Self::Err::InvalidSubdomain);
        }

        Ok(Self(value))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn valid_subdomain_from_str() {
        let subdomains = vec![
            ("www", true),
            ("www1", true),
            ("ww w", false),
            ("1wa1", true),
            ("tes!t", false),
            (".www", false),
            ("www.", false),
            (".www.", false),
            ("w@w", false),
            ("multiple.parts", true),
            ("multiple.part.", false),
            ("", false),
            ("'", false),
        ];

        for (value, expected) in subdomains {
            let subdomain = SubDomain::from_str(value);

            assert_eq!(subdomain.is_ok(), expected);
        }
    }

    #[test]
    fn valid_subdomain_matches_expected_str() {
        let subdomains = vec![
            ("WwW", "www"),
            ("wwW1", "www1"),
            ("TESTING.sub", "testing.sub"),
            ("TeStInG", "testing"),
        ];

        for (value, expected) in subdomains {
            let subdomain = SubDomain::from_str(value);

            assert!(subdomain.is_ok());

            if let Ok(subdomain) = subdomain {
                assert_eq!(subdomain.to_string(), expected);
            }
        }
    }
}
