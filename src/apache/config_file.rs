use std::{path::PathBuf, error::Error, fmt::Display, fs::File};

use crate::{domain::Domain, configuration_file::ConfigurationFile};

#[derive(Debug)]
pub enum ConfigError {
    FileSaving,
    InvalidPath,
    FileExists,
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FileSaving => write!(f,"Apache Configuration file could not be written to disk"),
            Self::InvalidPath => write!(f,"An invalid path was given for an Apache configuration file"),
            Self::FileExists => write!(f,"Apache configuration file already exists"),
        }    
    }
}

impl Error for ConfigError {}


impl ConfigFile {
    pub fn chown_to_www(domain: &Domain) -> Result<(), ConfigError> {
        Self::_chown_to_www(domain, ConfigError::FileSaving)
    }

    pub fn create(domain: &Domain) -> Result<File, ConfigError> {
        Self::_create(
            domain,
            ConfigError::FileExists,
            ConfigError::FileSaving,
        )
    }

    pub fn create_backup(domain: &Domain) -> Result<(), ConfigError> {
        Self::_create_backup(domain, ConfigError::FileSaving)
    }

    pub fn append(domain: &Domain) -> Result<File, ConfigError> {
        Self::_append(domain, ConfigError::InvalidPath)
    }
}

pub struct ConfigFile;

impl<'a> ConfigurationFile<'a> for ConfigFile {
    const SITES_AVAILABLE: &'a str = "/etc/apache/sites-available";

    fn server_name(domain: &Domain) -> String {
        format!("ServerName {domain}")
    }
}


#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::domain::Domain;

    use crate::apache::config_file::ConfigFile;

    use crate::configuration_file::ConfigurationFile;

    #[test]
    fn find_domain_without_subdomain_file() {
        let domains = vec![
            (Domain::new("example", "com", None), true),
            (Domain::new("example", "com", Some("www")), false),
            (Domain::new("Example", "com", None), true),
            (Domain::new("example", "COM", None), true),
            (Domain::new("www", "example", None), false),
        ];

        let haystack = r#"
        <VirtualHost *:443>
            ServerName example.com
            SSLEngine on
            SSLCertificateFile "/path/to/example.com.cert"
            SSLCertificateKeyFile "/path/to/example.com.key"
        </VirtualHost>"#;

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
        let haystack = r#"
        <VirtualHost *:443>
            ServerName www.example.com
            SSLEngine on
            SSLCertificateFile "/path/to/www.example.com.cert"
            SSLCertificateKeyFile "/path/to/www.example.com.key"
        </VirtualHost>"#;

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
    fn find_domain_with_commented_lines() {
        let domains = vec![
            (Domain::new("example", "com", None), false),
            (Domain::new("example", "com", Some("www")), true),
            (Domain::new("Example", "com", None), false),
            (Domain::new("example", "COM", None), false),
            (Domain::new("www", "example", None), false),
        ];

        let haystack = "
            #ServerName example.com
            ServerName www.example.com
            #ServerName www.example
            #ServerName example.COM";

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

        let expected = PathBuf::from("/etc/apache/sites-available/example.com.conf");

        let file_path = ConfigFile::file_path(&domain);

        assert_eq!(file_path, expected);
    }

    #[test]
    fn config_file_path_with_subdomain() {
        let domain = Domain::new_unchecked("example", "com", Some("www"));

        let expected = PathBuf::from("/etc/apache/sites-available/example.com.conf");

        let file_path = ConfigFile::file_path(&domain);

        assert_eq!(file_path, expected);
    }

    #[test]
    fn backup_file_path() {
        let domain = Domain::new_unchecked("example", "com", None);

        let expected = PathBuf::from("/etc/apache/sites-available/example.com.conf.bak");

        let backup_path = ConfigFile::backup_path(&domain);

        assert_eq!(backup_path, expected);
    }

    #[test]
    fn backup_file_path_with_subdomain() {
        let domain = Domain::new_unchecked("example", "com", Some("www"));

        let expected = PathBuf::from("/etc/apache/sites-available/example.com.conf.bak");

        let backup_path = ConfigFile::backup_path(&domain);

        assert_eq!(backup_path, expected);
    }
}
