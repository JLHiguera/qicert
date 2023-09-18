#![cfg(unix)]

mod sites;
mod domain;
mod webroot;
mod http_config;
mod linker;
mod certer;
mod config_file;
mod configurator;
mod nginx;

use std::error::Error;

use crate::domain::Domain;
use crate::configurator::Configurator;

fn main() -> Result<(), Box<dyn Error>>{
    let name = get_domain();

    //let domain = Domain::try_from(name.as_str())?;

    let tld = get_tld();

    let subdomain = get_subdomain();

    let domain = Domain::new(name, tld, subdomain)?;

    Configurator::append_or_create(&domain)?;

    Ok(())
}

fn get_domain() -> String {
    match std::env::args().nth(1) {
        Some(domain) => domain,
        None => panic!("Domain name is missing")
    }
}

fn get_tld() -> String {
    match std::env::args().nth(2) {
        Some(tld) => tld,
        None => panic!("tld is missing"),
    }
}

fn get_subdomain() -> Option<String> {
    std::env::args().nth(3)
}