use std::{error::Error, fs::File, io::Write};

use crate::{certer::Certer, domain::Domain, configuration_file::ConfigurationFile, apache::config_file::ConfigError};

use super::{Apache, config_file::ConfigFile, http_config::HttpConfig};

pub struct Configurator;

impl Configurator {
    pub fn create(domain: &Domain) -> Result<(), Box<dyn Error>> {
        let mut file = Self::create_file(domain)?;

        Self::add_well_known(&mut file, domain)?;

        Apache::reload()?;
        Certer::run(domain)?;
        ConfigFile::truncate_file(&mut file)?;
        Self::add_redirect_and_https(&mut file, domain)?;

        Apache::reload()?;

        Ok(())
    }

    fn create_file(domain: &Domain) -> Result<File, Box<dyn Error>> {
        if ConfigFile::file_exists(domain) {
            return Err(ConfigError::InvalidPath)?;
        }

        let file = ConfigFile::create(domain)?;

        ConfigFile::chown_to_www(domain)?;
        
        Ok(file)
    }

    fn add_well_known(file: &mut File, domain: &Domain) -> Result<(), Box<dyn Error>> {
        let server_block = HttpConfig::http_well_known(domain);

        writeln!(file, "{server_block}")?;

        Ok(())
    }

    fn add_redirect(file: &mut File, domain: &Domain) -> Result<(), Box<dyn Error>> {
        let redirect_block = HttpConfig::http_redirect(domain);

        writeln!(file, "{redirect_block}")?;

        Ok(())
    }

    fn add_https(file: &mut File, domain: &Domain) -> Result<(), Box<dyn Error>> {
        let https_block = HttpConfig::https_content(domain);

        writeln!(file, "{https_block}")?;

        Ok(())
    }

    fn add_redirect_and_https(file: &mut File, domain: &Domain) -> Result<(), Box<dyn Error>> {
        Self::add_redirect(file, domain)?;

        Self::add_https(file, domain)?;

        Ok(())
    }
}