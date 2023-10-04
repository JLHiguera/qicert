use std::{path::PathBuf, error::Error, fs::File, io::SeekFrom};

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

    fn file_exists(domain: &Domain) -> bool {
        let conf_path = Self::file_path(domain);

        conf_path.exists() || conf_path.is_file()
    }

    fn _chown_to_www<E: Error>(domain: &Domain, err: E) -> Result<(), E> {
        use std::process::Command;

        let conf_file_path = Self::file_path(domain);

        if !conf_file_path.exists() {
            return Err(err);
        }

        Command::new("chown")
            .arg("www-data:www-data")
            .arg(conf_file_path)
            .spawn()
            .and_then(|mut c| c.wait())
            .map_err(|_| err)?;

        Ok(())
    }

    fn _create<E: Error>(domain: &Domain, found_err: E, saving_err: E) -> Result<File, E> {
        if Self::file_exists(domain) {
            return Err(found_err);
        }

        let conf_path = Self::file_path(domain);

        let file = File::create(conf_path).map_err(|_| saving_err)?;

        Ok(file)
    }

    fn _create_backup<E: Error>(domain: &Domain, saving_err: E) -> Result<(), E> {
        let file_path = Self::file_path(domain);        
        let backup_path = Self::backup_path(domain);

        std::fs::copy(file_path, backup_path).map_err(|_| saving_err)?;

        Ok(())
    }

    fn truncate_file(file: &mut File) -> Result<(), Box<dyn Error>> {
        use std::io::Seek;

        file.set_len(0)?;
        
        file.seek(SeekFrom::End(0))?;

        Ok(())
    }

    fn _append<E: Error>(domain: &Domain, err: E) -> Result<File, E> {
        let conf_path = Self::file_path(domain);

        let file = std::fs::OpenOptions::new()
            .append(true)
            .read(true)
            .open(conf_path)
            .map_err(|_| err)?;

        Ok(file)
    }
    
    fn server_name(domain: &Domain) -> String;
}