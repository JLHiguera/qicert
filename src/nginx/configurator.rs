use std::{
    error::Error,
    fs::File,
    io::{Read, Write},
};

use crate::{nginx::Nginx, configuration_file::ConfigurationFile, webserver::WebServer};
use crate::nginx::config_file::{ConfigError, ConfigFile};
use crate::nginx::http_config::HttpConfig;
use crate::nginx::linker::Linker;

use crate::{
    certer::Certer,
    domain::Domain,
    webroot::WebRoot,
};

pub struct Configurator;

impl Configurator {
    fn create_file_and_link(domain: &Domain) -> Result<File, Box<dyn Error>> {
        if ConfigFile::file_exists(domain) {
            return Err(ConfigError::InvalidPath)?;
        }

        let file = ConfigFile::create(domain)?;

        ConfigFile::chown_to_www(domain)?;

        let message = match Linker::create(domain) {
            Ok(_) => "Link created",
            Err(ConfigError::SymlinkExists) => "Link exists. Skipping",
            Err(ConfigError::Linking) => "Missing permissions",
            _ => panic!("Unexpected error!"),
        };

        println!("{message}");

        Ok(file)
    }

    fn add_well_known(file: &mut File, domain: &Domain) -> Result<(), Box<dyn Error>> {
        let server_block = HttpConfig::http_well_known(domain);

        writeln!(file, "{server_block}").map_err(|_| ConfigError::FileSaving)?;

        Ok(())
    }

    fn add_redirect(file: &mut File, domain: &Domain) -> Result<(), Box<dyn Error>> {
        let redirect_block = HttpConfig::http_redirect_content(domain);

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

    fn create(domain: &Domain) -> Result<(), Box<dyn Error>> {
        let mut file = Self::create_file_and_link(domain)?;

        Self::add_well_known(&mut file, domain)?;

        WebRoot::create_and_set_chown(domain)?;
        Nginx::check_and_reload()?;
        Certer::run(domain)?;
        ConfigFile::truncate_file(&mut file)?;
        Self::add_redirect_and_https(&mut file, domain)?;

        Nginx::check_and_reload()?;

        Ok(())
    }

    fn panic_if_missing_nginx_or_certbot() {
        if !Self::are_nginx_and_certbot_installed() {
            std::panic::set_hook(Box::new(|_| {
                println!("Nginx or Certbot are missing. Shutting down.");
            }));

            panic!()
        }
    }

    pub fn append_or_create(domain: &Domain) -> Result<(), Box<dyn Error>> {
        Self::panic_if_missing_nginx_or_certbot();

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

            Nginx::check_and_reload()?;

            Certer::run(domain)?;

            ConfigFile::truncate_file(&mut file)?;

            file.write_all(content_backup.as_bytes())?;

            Self::add_redirect_and_https(&mut file, domain)?;

            Nginx::check_and_reload()?;
        }

        Ok(())
    }

    pub fn are_nginx_and_certbot_installed() -> bool {
        Nginx::is_installed() && Certer::is_installed()
    }
}
