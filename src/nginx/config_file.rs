use std::{error::Error, fmt::Display, fs};

use crate::{configuration_file::ConfigurationFile, domain::Domain};

#[derive(Debug)]
pub enum ConfigError {
    Linking,
    FileSaving,
    InvalidPath,
    SymlinkExists,
    FileExists,
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Linking => write!(f, "Configuration file could not be soft linked"),
            Self::FileSaving => write!(f, "Configuration file could not be written to disk"),
            Self::InvalidPath => write!(f, "An invalid path was given for a configuration file"),
            Self::SymlinkExists => write!(f, "Symlink already exists"),
            Self::FileExists => write!(f, "Configuration file already exists"),
        }
    }
}

impl Error for ConfigError {}

impl<'a> ConfigurationFile<'a> for ConfigFile {
    const SITES_AVAILABLE: &'a str = "/etc/nginx/sites-available";

    fn server_name(domain: &Domain) -> String {
        format!("server_name {};", domain)
    }
}

pub struct ConfigFile;

impl ConfigFile {
    pub fn chown_to_www(domain: &Domain) -> Result<(), ConfigError> {
        Self::_chown_to_www(domain, ConfigError::FileSaving)
    }

    pub fn create(domain: &Domain) -> Result<fs::File, ConfigError> {
        Self::_create(domain, ConfigError::FileExists, ConfigError::FileSaving)
    }

    pub fn create_backup(domain: &Domain) -> Result<(), ConfigError> {
        Self::_create_backup(domain, ConfigError::FileSaving)
    }

    pub fn append(domain: &Domain) -> Result<fs::File, ConfigError> {
        Self::_append(domain, ConfigError::InvalidPath)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn find_domain_without_subdomain_file() {
        let domains = vec![
            (Domain::new("example", "com", None), true),
            (Domain::new("example", "com", Some("www")), false),
            (Domain::new("Example", "com", None), true),
            (Domain::new("example", "COM", None), true),
            (Domain::new("www", "example", None), false),
        ];

        let haystack = "server { 
                server_name example.com;
        }";

        for (domain, expected) in domains {
            if let Ok(domain) = domain {
                assert_eq!(
                    ConfigFile::find_domain_in_str(haystack, &domain),
                    expected,
                    "domain: {domain}"
                );
            }
        }
    }

    #[test]
    fn find_domain_with_subdomain_in_file() {
        let domains = vec![
            (Domain::new("example", "com", None), false),
            (Domain::new("example", "com", Some("www")), true),
            (Domain::new("Example", "com", None), false),
            (Domain::new("example", "COM", None), false),
            (Domain::new("www", "example", None), false),
        ];

        let haystack = "server {
            server_name www.example.com;
        }";

        for (domain, expected) in domains {
            if let Ok(domain) = domain {
                assert_eq!(
                    ConfigFile::find_domain_in_str(haystack, &domain),
                    expected,
                    "domain: {domain}"
                );
            }
        }
    }

    #[test]
    fn find_domain_with_commencted_lines() {
        let domains = vec![
            (Domain::new("example", "com", None), false),
            (Domain::new("example", "com", Some("www")), true),
            (Domain::new("Example", "com", None), false),
            (Domain::new("example", "COM", None), false),
            (Domain::new("www", "example", None), false),
        ];

        let haystack = "server {
            #server_name example.com;
            server_name www.example.com;
            #server_name www.example;
            #server_name example.COM;
        }";

        for (domain, expected) in domains {
            if let Ok(domain) = domain {
                assert_eq!(
                    ConfigFile::find_domain_in_str(haystack, &domain),
                    expected,
                    "domain: {domain}"
                );
            }
        }
    }

    #[test]
    fn config_file_path_without_subdomain() {
        let domain = Domain::new_unchecked("example", "com", None);

        let expected = PathBuf::from("/etc/nginx/sites-available/example.com.conf");

        let file_path = ConfigFile::file_path(&domain);

        assert_eq!(file_path, expected);
    }

    #[test]
    fn config_file_path_with_subdomain() {
        let domain = Domain::new_unchecked("example", "com", Some("www"));

        let expected = PathBuf::from("/etc/nginx/sites-available/example.com.conf");

        let file_path = ConfigFile::file_path(&domain);

        assert_eq!(file_path, expected);
    }

    #[test]
    fn backup_file_path() {
        let domain = Domain::new_unchecked("example", "com", None);

        let expected = PathBuf::from("/etc/nginx/sites-available/example.com.conf.bak");

        let backup_path = ConfigFile::backup_path(&domain);

        assert_eq!(backup_path, expected);
    }

    #[test]
    fn backup_file_path_with_subdomain() {
        let domain = Domain::new_unchecked("example", "com", Some("www"));

        let expected = PathBuf::from("/etc/nginx/sites-available/example.com.conf.bak");

        let backup_path = ConfigFile::backup_path(&domain);

        assert_eq!(backup_path, expected);
    }
}
