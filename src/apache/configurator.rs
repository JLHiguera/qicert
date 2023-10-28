use std::{error::Error, fs::File, io::{Write, Read}};

use crate::{certer::Certer, domain::Domain, configuration_file::ConfigurationFile, apache::config_file::ConfigError, webserver::WebServer, webroot::WebRoot};

use super::{Apache, config_file::ConfigFile, http_config::HttpConfig};

pub struct Configurator;

impl Configurator {
    pub fn create(domain: &Domain) -> Result<(), Box<dyn Error>> {
        let mut file = Self::create_file(domain)?;
        Self::add_well_known(&mut file, domain)?;

        WebRoot::create_and_set_chown(domain)?;

        Apache::enable_site(domain)?;
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

    fn panic_if_missing_apache_or_certbot() {
        if !Self::are_apache_and_certbot_installed() {
            std::panic::set_hook(Box::new(|_| {
                println!("Apache or Certbot are missing. Shutting down.");
            }));

            panic!()
        }
    }

    pub fn append_or_create(domain: &Domain) -> Result<(), Box<dyn Error>> {
        Self::panic_if_missing_apache_or_certbot();

        if !ConfigFile::file_exists(domain) {
            return Self::create(domain);
        }

        Self::append(domain)
    }

    fn append(domain: &Domain) -> Result<(), Box<dyn Error>> {
        ConfigFile::create_backup(domain)?;

        let mut file = ConfigFile::append(domain)?;

        let content_backup = {
            let mut tmp = String::new();

            file.read_to_string(&mut tmp)?;

            tmp
        };


        if !ConfigFile::find_domain_in_str(content_backup.as_str(), domain) {
            Self::add_well_known(&mut file, domain)?;
            match WebRoot::create_and_set_chown(domain) {
                Ok(_) => println!("Webroot created for {domain}"),
                Err(e) => println!("{e} error for {domain}"),
            };

            Apache::enable_site(domain)?;
            Apache::reload()?;

            Certer::run(domain)?;

            ConfigFile::truncate_file(&mut file)?;
            file.write_all(content_backup.as_bytes())?;

            Self::add_redirect_and_https(&mut file, domain)?;

            Apache::reload()?;
        }

        Ok(())
    }

    pub fn are_apache_and_certbot_installed() -> bool {
        Apache::is_installed() && Certer::is_installed()
    }
}