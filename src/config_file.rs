use std::{
    error::Error,
    fmt::Display,
    fs,
    io::{Seek, SeekFrom},
    path::PathBuf,
};

use crate::{domain::Domain, sites::Sites};

#[derive(Debug)]
pub enum ConfigError {
    Linking,
    FileSaving,
    InvalidPath,
    Appending,
    SymlinkExists,
    FileExists,
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Error for ConfigError {}

pub struct ConfigFile;

impl ConfigFile {
    pub fn find_domain_in_str<S: AsRef<str>>(haystack: S, domain: &Domain) -> bool {
        fn inner(haystack: &str, domain: &Domain) -> bool {
            let needle = format!("server_name {};", domain);

            haystack
                .lines()
                .map(|l| l.trim())
                .filter(|l| !l.contains('#'))
                .any(|l| l.contains(needle.as_str()))
        }

        inner(haystack.as_ref(), domain)
    }

    pub fn find_domain_in_file(domain: &Domain) -> bool {
        let haystack = Self::copy_to_string(domain).unwrap();

        Self::find_domain_in_str(haystack, domain)
    }

    fn file_path(domain: &Domain) -> PathBuf {
        let (mut sites_available, _) = Sites::paths();

        let file_name = Self::file_name(domain);

        sites_available.push(file_name);

        sites_available
    }

    pub fn file_name(domain: &Domain) -> String {
        format!("{}.{}.conf", domain.get_name(), domain.get_tld())
    }

    pub fn chown_to_www(domain: &Domain) -> Result<(), ConfigError> {
        use std::process::Command;

        let conf_file_path = Self::file_path(domain);

        if !conf_file_path.exists() {
            return Err(ConfigError::FileSaving);
        }

        Command::new("chown")
            .arg("www-data:www-data")
            .arg(conf_file_path)
            .spawn()
            .and_then(|mut p| p.wait())
            .map_err(|_| ConfigError::FileSaving)?;

        Ok(())
    }

    pub fn file_exists(domain: &Domain) -> bool {
        let conf_path = Self::file_path(domain);

        conf_path.exists() || conf_path.is_file()
    }

    // fn is_empty(domain: &Domain) -> bool {
    //     let conf_path = Self::file_path(domain);

    //     let meta = fs::metadata(conf_path);

    //     match meta {
    //         Ok(meta) => meta.len() == 0,

    //     }

    //     let metadata = fs::metadata(conf_path)
    //         .map_err(|_| ConfigError::InvalidPath)?;

    //     Ok(metadata.len() == 0)
    // }

    pub fn create(domain: &Domain) -> Result<fs::File, ConfigError> {
        if Self::file_exists(domain) {
            return Err(ConfigError::FileExists);
        }

        let conf_path = Self::file_path(domain);

        let file = fs::File::create(conf_path).map_err(|_| ConfigError::FileSaving)?;

        Ok(file)
    }

    pub fn create_backup(domain: &Domain) -> Result<(), ConfigError> {
        let file_path = Self::file_path(domain);

        let backup_path = Self::backup_path(domain);

        std::fs::copy(file_path, backup_path).map_err(|_| ConfigError::FileSaving)?;

        Ok(())
    }

    fn backup_path(domain: &Domain) -> PathBuf {
        let file_path = Self::file_path(domain);

        file_path.with_extension("conf.bak")
    }

    fn open_or_create(domain: &Domain) -> Result<fs::File, ConfigError> {
        if !Self::file_exists(domain) {
            return Self::create(domain);
        }

        Self::open(domain)
    }

    pub fn truncate_file(file: &mut fs::File) -> Result<(), Box<dyn Error>> {
        file.set_len(0)?;

        file.seek(SeekFrom::End(0))?;

        Ok(())
    }

    pub fn append_or_create(domain: &Domain) -> Result<fs::File, ConfigError> {
        if !Self::file_exists(domain) {
            return Self::create(domain);
        }

        let file = Self::append(domain)?;

        Ok(file)
    }

    pub fn append(domain: &Domain) -> Result<fs::File, ConfigError> {
        let conf_path = Self::file_path(domain);

        let file = fs::OpenOptions::new()
            .append(true)
            .read(true)
            .open(conf_path)
            .map_err(|_| ConfigError::InvalidPath)?;

        Ok(file)
    }

    pub fn open(domain: &Domain) -> Result<fs::File, ConfigError> {
        let conf_path = Self::file_path(domain);

        let file = fs::File::open(conf_path).map_err(|_| ConfigError::InvalidPath)?;

        Ok(file)
    }

    fn copy_to_string(domain: &Domain) -> Result<String, ConfigError> {
        let content =
            fs::read_to_string(Self::file_path(domain)).map_err(|_| ConfigError::InvalidPath)?;

        Ok(content)
    }
}

#[cfg(test)]
mod test {
    use super::*;

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
