use crate::{domain::Domain, webroot::WebRoot};

pub struct HttpConfig;

impl HttpConfig {
    pub fn http_redirect_content(domain: &Domain) -> String {
        let domain_name = domain.to_string();

        format!("server {{
            listen 80;
    
            server_name {domain_name};
    
            return 301 https://{domain_name}$request_uri;
    }}")
    }
    
    fn server_name(domain: &Domain) -> String {
        format!("server_name {}", domain)
    }

    pub fn http_well_known(domain: &Domain) -> String {
        let server_name = Self::server_name(domain);

        let root = WebRoot::build_path_string(domain);

        format!("server {{
            listen 80;
    
            {server_name};
    
            location ^~ /.well-known/acme-challenge/ {{
                root {root};
                allow all;
                default_type \"text/plain\";
            }}
    }}")
    }
    
    pub fn https_content(domain: &Domain) -> String {
        let server_name = Self::server_name(domain);
        
        let root = format!("root {}", WebRoot::build_path_string(domain));

        format!(r##"server {{
            {server_name};
            listen 443 ssl;
        
            ssl_certificate /etc/letsencrypt/live/{domain}/fullchain.pem;
            ssl_certificate_key /etc/letsencrypt/live/{domain}/privkey.pem;
            ssl_trusted_certificate /etc/letsencrypt/live/{domain}/fullchain.pem;
        
            include /etc/letsencrypt/options-ssl-nginx.conf;
            ssl_dhparam /etc/letsencrypt/ssl-dhparams.pem;

            {root};
            index index.html;
            location / {{
                try_files $uri $uri/ =404;
            }}
    }}"##)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn acme_challenge_block_no_subdomain() {
        let expected = format!("server {{
            listen 80;
    
            server_name example.com;
    
            location ^~ /.well-known/acme-challenge/ {{
                root /var/www/example.com/public;
                allow all;
                default_type \"text/plain\";
            }}
    }}");

    let domain = Domain::try_from("example.com").unwrap();

    let challenge_block = HttpConfig::http_well_known(&domain);

    assert_eq!(challenge_block, expected);
    }

    #[test]
    fn acme_challenge_block_with_subdomain() {
        let expected = format!("server {{
            listen 80;
    
            server_name test.example.com;
    
            location ^~ /.well-known/acme-challenge/ {{
                root /var/www/test.example.com/public;
                allow all;
                default_type \"text/plain\";
            }}
    }}");

        let domain = Domain::try_from("test.example.com").unwrap();

        let challenge_block = HttpConfig::http_well_known(&domain);

        assert_eq!(challenge_block, expected);
    }

    #[test]
    fn https_block_no_subdomain() {
        let expected = format!(r##"server {{
            server_name example.com;
            listen 443 ssl;
        
            ssl_certificate /etc/letsencrypt/live/example.com/fullchain.pem;
            ssl_certificate_key /etc/letsencrypt/live/example.com/privkey.pem;
            ssl_trusted_certificate /etc/letsencrypt/live/example.com/fullchain.pem;
        
            include /etc/letsencrypt/options-ssl-nginx.conf;
            ssl_dhparam /etc/letsencrypt/ssl-dhparams.pem;

            root /var/www/example.com/public;
            index index.html;
            location / {{
                try_files $uri $uri/ =404;
            }}
    }}"##);

        let domain = Domain::try_from("example.com").unwrap();

        let http_block = HttpConfig::https_content(&domain);

        assert_eq!(http_block, expected);
        
    }

    #[test]
    fn https_block_with_subdomain() {
        let expected = format!(r##"server {{
            server_name www.example.com;
            listen 443 ssl;
        
            ssl_certificate /etc/letsencrypt/live/www.example.com/fullchain.pem;
            ssl_certificate_key /etc/letsencrypt/live/www.example.com/privkey.pem;
            ssl_trusted_certificate /etc/letsencrypt/live/www.example.com/fullchain.pem;
        
            include /etc/letsencrypt/options-ssl-nginx.conf;
            ssl_dhparam /etc/letsencrypt/ssl-dhparams.pem;

            root /var/www/www.example.com/public;
            index index.html;
            location / {{
                try_files $uri $uri/ =404;
            }}
    }}"##);

        let domain = Domain::try_from("www.example.com").unwrap();

        let http_block = HttpConfig::https_content(&domain);

        assert_eq!(http_block, expected);
        
    }

    #[test]
    fn http_redirect_block_no_subdomain() {
        let expected = format!("server {{
            listen 80;
    
            server_name example.com;
    
            return 301 https://example.com$request_uri;
    }}");

        let domain = Domain::try_from("example.com").unwrap();

        let redirect_block = HttpConfig::http_redirect_content(&domain);

        assert_eq!(redirect_block, expected);
    }


    #[test]
    fn http_redirect_block_with_subdomain() {
        let expected = format!("server {{
            listen 80;
    
            server_name www.example.com;
    
            return 301 https://www.example.com$request_uri;
    }}");

        let domain = Domain::try_from("www.example.com").unwrap();

        let redirect_block = HttpConfig::http_redirect_content(&domain);

        assert_eq!(redirect_block, expected);
    }
}