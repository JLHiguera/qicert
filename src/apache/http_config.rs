
#[cfg(test)]
mod test {
    use crate::domain::Domain;

    use super::*;

    #[test]
    fn http_well_known() {
        let expected = 
        "<VirtualHost *:80>
            ServerAdmin webmaster@localhost
            ServerName example.com
            DocumentRoot /var/www/.well-known/challenge
            ErrorLog ${APACHE_LOG_DIR}/error.log
            CustomLog ${APACHE_LOG_DIR}/access.log combined
        </VirtualHost>
        ".to_string();

        let domain = crate::Domain::new("example", "com", None).unwrap();

        let http_config = crate::apache::http_config::HttpConfig::http_well_known(&domain);

        assert_eq!(http_config, expected);
    }

    #[test]
    fn http_well_known_with_short_subdomain() {
        let expected = 
        "<VirtualHost *:80>
            ServerAdmin webmaster@localhost
            ServerName test.example.com
            DocumentRoot /var/www/.well-known/challenge
            ErrorLog ${APACHE_LOG_DIR}/error.log
            CustomLog ${APACHE_LOG_DIR}/access.log combined
        </VirtualHost>
        ".to_string();

        let domain = crate::Domain::new("example", "com", Some("test")).unwrap();

        let http_config = crate::apache::http_config::HttpConfig::http_well_known(&domain);

        assert_eq!(http_config, expected);
    }

    #[test]
    fn http_well_known_with_long_subdomain() {
        let expected = 
        "<VirtualHost *:80>
            ServerAdmin webmaster@localhost
            ServerName test1.staging1.example.com
            DocumentRoot /var/www/.well-known/challenge
            ErrorLog ${APACHE_LOG_DIR}/error.log
            CustomLog ${APACHE_LOG_DIR}/access.log combined
        </VirtualHost>
        ".to_string();

        let domain = crate::Domain::new("example", "com", Some("test1.staging1")).unwrap();

        let http_config = crate::apache::http_config::HttpConfig::http_well_known(&domain);

        assert_eq!(http_config, expected);
    }



    #[test]
    fn http_redirect_to_https() {
        let expected = 
            "<VirtualHost *:80> 
            ServerName example.com
        
            Redirect permanent / https://example.com/
        </VirtualHost>
      ".to_string();

        let domain = Domain::new("example", "com", None).unwrap();

        let http_config = crate::apache::http_config::HttpConfig::http_redirect(&domain);

        assert_eq!(http_config, expected);
    }

    #[test]
    fn http_redirect_to_https_with_short_subdomain() {
        let expected = 
            "<VirtualHost *:80> 
            ServerName test.example.com
        
            Redirect permanent / https://test.example.com/
        </VirtualHost>
      ".to_string();

        let domain = Domain::new("example", "com", Some("test")).unwrap();

        let http_config = crate::apache::http_config::HttpConfig::http_redirect(&domain);

        assert_eq!(http_config, expected);
    }

    #[test]
    fn http_redirect_to_https_with_long_subdomain() {
        let expected = 
            "<VirtualHost *:80> 
            ServerName test1.staging1.example.com
        
            Redirect permanent / https://test.example.com/
        </VirtualHost>
      ".to_string();

        let domain = Domain::new("example", "com", Some("test1.staging1")).unwrap();

        let http_config = crate::apache::http_config::HttpConfig::http_redirect(&domain);

        assert_eq!(http_config, expected);
    }



    #[test]
    fn https_block() {
        let expected = 
            "<VirtualHost *:443>
            ServerName example.com
        
            Protocols h2 http/1.1
        
            SSLCertificateFile /etc/letsencrypt/live/example.com/fullchain.pem
            SSLCertificateKeyFile /etc/letsencrypt/live/example.com/privkey.pem
        
            # Other Apache Configuration
        
        </VirtualHost>
      ".to_string();

      let domain = Domain::new("example", "com", None).unwrap();

      let http_config = crate::apache::http_config::HttpConfig::https(&domain);

      assert_eq!(http_config, expected);
    }

    #[test]
    fn https_block_with_short_subdomain() {
        let expected = 
            "<VirtualHost *:443>
            ServerName test.example.com
        
            Protocols h2 http/1.1
        
            SSLCertificateFile /etc/letsencrypt/live/test.example.com/fullchain.pem
            SSLCertificateKeyFile /etc/letsencrypt/live/test.example.com/privkey.pem
        
            # Other Apache Configuration
        
        </VirtualHost>
      ".to_string();

      let domain = Domain::new("example", "com", Some("test")).unwrap();

      let http_config = crate::apache::http_config::HttpConfig::https(&domain);

      assert_eq!(http_config, expected);
    }

    #[test]
    fn https_block_with_long_subdomain() {
        let expected = 
            "<VirtualHost *:443>
            ServerName test1.staging1.example.com
        
            Protocols h2 http/1.1
        
            SSLCertificateFile /etc/letsencrypt/live/test.example.com/fullchain.pem
            SSLCertificateKeyFile /etc/letsencrypt/live/test.example.com/privkey.pem
        
            # Other Apache Configuration
        
        </VirtualHost>
      ".to_string();

      let domain = Domain::new("example", "com", Some("test1.staging1")).unwrap();

      let http_config = crate::apache::http_config::HttpConfig::https(&domain);

      assert_eq!(http_config, expected);
    }
}