use std::os::unix::fs as unix_fs;

use crate::{
    config_file::{ConfigError, ConfigFile},
    domain::Domain,
    sites::Sites,
};

pub struct Linker;

impl Linker {
    pub fn exists(domain: &Domain) -> bool {
        let file_name = domain.conf_file_name();

        let (_, mut site_symlink) = Sites::paths();

        site_symlink.push(file_name);

        site_symlink.exists() && site_symlink.is_symlink()
    }

    pub fn create(domain: &Domain) -> Result<(), ConfigError> {
        if Self::exists(domain) {
            return Err(ConfigError::SymlinkExists);
        }

        let file_name = ConfigFile::file_name(domain);

        let (mut sites_available, mut sites_enabled) = Sites::paths();

        sites_available.push(file_name.as_str());
        sites_enabled.push(file_name.as_str());

        unix_fs::symlink(sites_available, sites_enabled).map_err(|_| ConfigError::Linking)?;

        Ok(())
    }
}
