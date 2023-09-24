mod domain_name;
mod subdomain;
mod tld;

use std::{fmt::Display, error::Error, str::FromStr};

use crate::domain::{domain_name::DomainName, tld::Tld, subdomain::SubDomain};

#[derive(Debug, PartialEq, Eq)]
pub struct Domain {
    name: String,
    tld: String,
    subdomain: Option<String>,
}
#[derive(Debug)]
pub enum DomainError {
    MissingTld,
    InvalidName,
    InvalidSubdomain,
    InvalidTld,
}

impl Display for DomainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //write!(f, "{}", self)
        match self {
            Self::MissingTld => write!(f, "The Domain has no TLD"),
            Self::InvalidName => write!(f, "Invalid domain name"),
            Self::InvalidSubdomain => write!(f, "The subdomain given is invalid"),
            Self::InvalidTld => write!(f, "The TLD is invalid"),
        }
    }
}

impl Error for DomainError {}

impl Domain {
    pub fn new<S: AsRef<str>>(name: S, tld: S, subdomain: Option<&str>) -> Result<Self, DomainError> {
        fn inner(name: &str, tld: &str, subdomain: Option<&str>) -> Result<Domain, DomainError> {
            if ! DomainName::is_valid(name) {
                return Err(DomainError::InvalidName);
            }

            if ! Tld::is_valid(tld) {
                return Err(DomainError::InvalidTld);
            }

            let subdomain = match subdomain {
                Some(sub) if SubDomain::is_valid(&sub) => Some(sub.to_owned()),
                Some(_) => return Err(DomainError::InvalidSubdomain),
                None => None,
            };

            let domain = Domain {
                name: name.to_owned(), 
                tld: tld.to_owned(), 
                subdomain: subdomain,
            };

            Ok(domain)
        }
        
        inner(name.as_ref(), tld.as_ref(), subdomain)
    }

    #[cfg(test)]
    pub fn new_unchecked<S: AsRef<str>>(name: S, tld: S, subdomain: Option<&str>) -> Self {
        Self::new(name.as_ref(), tld.as_ref(), subdomain.as_deref()).unwrap()
    }

    pub fn get_name(&self) -> String {
        self.name.to_owned()
    }

    pub fn get_tld(&self) -> String {
        self.tld.to_owned()
    }

    fn name_with_tld(&self) -> String {
        format!("{}.{}", self.name, self.tld)
    }

    fn has_subdomain(&self) -> bool {
        self.subdomain.is_some()
    }

    fn get_subdomain(&self) -> Option<String> {
        self.subdomain.clone()
    }

    fn is_valid_char(char: char) -> bool {
        matches!(char, 'a'..='z' | '0'..='9'| '.')
    }
    
    pub fn conf_file_name(&self) -> String {
        format!("{}.{}.conf", self.name, self.tld)
    }
}

impl std::ops::Add<Tld> for DomainName {
    type Output = Domain;

    fn add(self, rhs: Tld) -> Self::Output {
        let name = self;

        Self::Output {
            name: name.get_name(),
            tld: rhs.to_string(),
            subdomain: None,
        }
    }
}

impl Display for Domain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.subdomain {
            Some(subdomain) => write!(f, "{}.{}.{}", subdomain, self.name, self.tld),
            None => write!(f, "{}.{}", self.name, self.tld),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn build_from_parts() {
        let expected = Domain::new("example", "com", None);

        let name = DomainName::from_str("example").unwrap();

        let tld = Tld::from_str("com").unwrap();

        let domain = name + tld;

        assert_eq!(domain.to_string(), "example.com");

        assert!(expected.is_ok());

        if let Ok(expected) = expected {
            assert_eq!(expected, domain);
        }
    }

    #[test]
    fn build_from_parts_str() {
        let expected = Domain::new_unchecked("example", "com", None);

        let domain = Domain::new("example", "com", None);

        assert!(domain.is_ok());
        
        if let Ok(domain) = domain {
            assert_eq!(domain, expected);
        }
    }

    #[test]
    fn build_from_parts_str_without_subdomain() {
        let domain_parts = vec![
            ("example", "com", true),
            ("example1", "com.mx", true),
            ("example", "edu.es", true),
            ("example", "com.mx.", false),
            ("", "com.mx", false),
            (".", "com", false),
            ("..", "com", false),
            (".", ".", false),
            ("example", "", false),
            ("example ", "com", false),
            ("example", ".com", false),
            ("example", "com ", false),
            ("example", "com.", false),
            ("example.", "com", false),
            ("example.", "com", false),
            (".example", "com", false),
            ("example-me", "com", true),
            ("-example", "com", false),
            ("example-", "com", false),
        ];

        for (domain_name, tld, expected) in domain_parts {
            let domain = Domain::new(domain_name, tld, None);

            assert_eq!(domain.is_ok(), expected, "domain tested was: {domain_name}, and {tld}");
        }
    }

    #[test]
    fn build_from_parts_str_with_subdomain() {
        let domain_parts = vec![
            ("example", "com", "test1", true),
            ("example", "com.mx", "", false),
            ("example", "com.mx", "staging01", true),
            ("example", "com.mx", "staging02.test", true),
            ("example", "net", "staging-02.test", true),
            ("example", "net", "staging02-", false),
        ];

        for (domain_name, tld, subdomain, expected) in domain_parts {
            let domain = Domain::new(domain_name, tld, Some(subdomain));

            assert_eq!(domain.is_ok(), expected, "name: {domain_name}, tld: {tld}, subdomain: {subdomain}");
        }
    }

    #[test]
    fn build_from_parts_str_with_subdomain_then_into_string() {
        let domain_parts = vec![
            ("example", "com", "test1", "test1.example.com"),
            ("example", "com.mx", "staging01", "staging01.example.com.mx"),
            ("example", "com.mx", "staging02.test", "staging02.test.example.com.mx"),
            ("example", "net", "staging-02.test", "staging-02.test.example.net"),
            ("example", "net", "staging-02.test", "staging-02.test.example.net"),
        ];

        for (domain_name, tld, subdomain, expected) in domain_parts {
            let domain = Domain::new(domain_name, tld, Some(subdomain));

            assert!(domain.is_ok());

            if let Ok(domain) = domain {
                assert_eq!(domain.to_string(), expected, "domain given: {domain},expected: {expected}");
            }
        }
    }

    #[test]
    fn build_from_struct_parts_without_subdomain() {
        let expected_domain = Domain::new_unchecked("example", "net", None);
        let expected_string: String = "example.net".into();

        let domain_name = DomainName::from_str("example");

        let tld = Tld::from_str("net");

        if let (Ok(domain_name), Ok(tld)) = (domain_name, tld) {
            let domain = Domain::from_parts(domain_name, tld, None);

            assert_eq!(domain, expected_domain);

            assert_eq!(domain.to_string(), expected_string);
        }
    }
    
    #[test]
    fn build_from_parts_with_subdomain() {
        let expected_domain = Domain::new_unchecked("example", "net", Some("www"));
        let expected_string: String = "www.example.net".into();

        let domain_name = DomainName::from_str("example");

        let tld = Tld::from_str("net");

        let subdomain = SubDomain::from_str("www");

        if let (Ok(domain_name), Ok(tld), Ok(subdomain)) = (domain_name, tld, subdomain) {
            assert_eq!(domain_name.to_string(), "example");

            assert_eq!(tld.to_string(), "net");

            let domain = Domain::from_parts(domain_name, tld, Some(subdomain));

            assert_eq!(domain, expected_domain);

            assert_eq!(domain.to_string(), expected_string);
        }
    }
}