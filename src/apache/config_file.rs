use std::path::PathBuf;

use crate::domain::Domain;

pub struct ConfigFile;


impl ConfigFile {
    const SITES_AVAILABLE: &str = "/etc/apache/sites-available";

    pub fn find_domain_in_str<S: AsRef<str>>(haystack: S, domain: &Domain) -> bool {
        fn inner(haystack: &str, domain: &Domain) -> bool {
            let needle = format!("ServerName {domain}");


            haystack
                .lines()
                .map(str::trim)
                .filter(|l| !l.contains('#'))
                .any(|l| l.ends_with(&needle))
        }
        

        inner(haystack.as_ref(), domain)
    }

    pub fn file_name(domain: &Domain) -> String {
        format!("{}.{}.conf", domain.get_name(), domain.get_tld())
    }

    fn file_path(domain: &Domain) -> PathBuf {
        let mut base_path = Self::path();

        let file_name = Self::file_name(domain);

        base_path.push(file_name);

        base_path
    }

    fn backup_path(domain: &Domain) -> PathBuf {
        Self::file_path(domain).with_extension("conf.bak")
    }

    pub fn path() -> PathBuf {
        PathBuf::from(Self::SITES_AVAILABLE)
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::domain::Domain;

    use crate::apache::config_file::ConfigFile;

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
