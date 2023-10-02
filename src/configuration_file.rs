use std::path::PathBuf;

use crate::domain::Domain;

pub(crate) trait ConfigurationFile<'a> {
    const SITES_AVAILABLE: &'a str;

    fn find_domain_in_str<S: AsRef<str>>(haystack: S, domain: &Domain) -> bool {
        let needle = Self::server_name(domain);

        let haystack = haystack.as_ref();

        haystack
            .lines()
            .map(str::trim)
            .filter(|l| !l.contains('#'))
            .any(|l| l.ends_with(&needle))        
    }

    fn sites_enabled_path() -> PathBuf {
        PathBuf::from(Self::SITES_AVAILABLE)
    }

    fn file_name(domain: &Domain) -> String {
        format!("{}.{}.conf", domain.get_name(), domain.get_tld())
    }

    fn file_path(domain: &Domain) -> PathBuf {
        let mut base_path = Self::sites_enabled_path();

        let file_name = Self::file_name(domain);

        base_path.push(file_name);

        base_path
    }

    fn backup_path(domain: &Domain) -> PathBuf {
        Self::file_path(domain).with_extension("conf.bak")
    }

    fn server_name(domain: &Domain) -> String;
}