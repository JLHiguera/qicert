mod domain_name;
mod subdomain;
mod tld;

use std::{error::Error, fmt::Display, str::FromStr};

use crate::domain::{domain_name::DomainName, subdomain::SubDomain, tld::Tld};

#[derive(Debug, PartialEq, Eq)]
pub struct Domain {
    name: DomainName,
    tld: Tld,
    subdomain: Option<SubDomain>,
}
#[derive(Debug)]
pub enum DomainError {
    MissingTld,
    InvalidName,
    InvalidSubdomain,
    InvalidTld,
    TldTooShort,
}

impl Display for DomainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingTld => write!(f, "The Domain has no TLD"),
            Self::TldTooShort => write!(f, "TLD given is too short"),
            Self::InvalidName => write!(f, "Invalid domain name"),
            Self::InvalidSubdomain => write!(f, "The subdomain given is invalid"),
            Self::InvalidTld => write!(f, "The TLD is invalid"),
        }
    }
}

impl Error for DomainError {}

impl Domain {
    pub fn new<S: AsRef<str>>(
        name: S,
        tld: S,
        subdomain: Option<&str>,
    ) -> Result<Self, DomainError> {
        fn inner(name: &str, tld: &str, subdomain: Option<&str>) -> Result<Domain, DomainError> {
            let name = DomainName::from_str(name)?;

            let tld = Tld::from_str(tld)?;

            let subdomain = match subdomain {
                Some(sub) => Some(SubDomain::from_str(sub)?),
                None => None,
            };

            let domain = Domain::from_parts(name, tld, subdomain);

            Ok(domain)
        }

        inner(name.as_ref(), tld.as_ref(), subdomain)
    }

    pub fn from_parts(name: DomainName, tld: Tld, subdomain: Option<SubDomain>) -> Self {
        Self {
            name,
            tld,
            subdomain,
        }
    }

    #[cfg(test)]
    pub fn new_unchecked<S: AsRef<str>>(name: S, tld: S, subdomain: Option<&str>) -> Self {
        Self::new(name.as_ref(), tld.as_ref(), subdomain).unwrap()
    }

    pub fn get_name(&self) -> DomainName {
        self.name.clone()
    }

    pub fn get_tld(&self) -> Tld {
        self.tld.clone()
    }

    pub fn conf_file_name(&self) -> String {
        format!("{}.{}.conf", self.name, self.tld)
    }
}

impl std::ops::Add<Tld> for DomainName {
    type Output = Domain;

    fn add(self, tld: Tld) -> Self::Output {
        let name = self;

        Self::Output {
            name,
            tld,
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

            assert_eq!(
                domain.is_ok(),
                expected,
                "domain tested was: {domain_name}, and {tld}"
            );
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

            assert_eq!(
                domain.is_ok(),
                expected,
                "name: {domain_name}, tld: {tld}, subdomain: {subdomain}"
            );
        }
    }

    #[test]
    fn build_from_parts_str_with_subdomain_then_into_string() {
        let domain_parts = vec![
            ("example", "com", "test1", "test1.example.com"),
            ("example", "com.mx", "staging01", "staging01.example.com.mx"),
            (
                "example",
                "com.mx",
                "staging02.test",
                "staging02.test.example.com.mx",
            ),
            (
                "example",
                "net",
                "staging-02.test",
                "staging-02.test.example.net",
            ),
            (
                "example",
                "net",
                "staging-02.test",
                "staging-02.test.example.net",
            ),
        ];

        for (domain_name, tld, subdomain, expected) in domain_parts {
            let domain = Domain::new(domain_name, tld, Some(subdomain));

            assert!(domain.is_ok());

            if let Ok(domain) = domain {
                assert_eq!(
                    domain.to_string(),
                    expected,
                    "domain given: {domain},expected: {expected}"
                );
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

        let subdomain = SubDomain::from_str("wwW");

        assert!(domain_name.is_ok());

        assert!(tld.is_ok());

        assert!(subdomain.is_ok());

        if let (Ok(domain_name), Ok(tld), Ok(subdomain)) = (domain_name, tld, subdomain) {
            assert_eq!(domain_name.to_string(), "example");

            assert_eq!(tld.to_string(), "net");

            let domain = Domain::from_parts(domain_name, tld, Some(subdomain));

            assert_eq!(domain, expected_domain);

            assert_eq!(domain.to_string(), expected_string);
        }
    }
}
