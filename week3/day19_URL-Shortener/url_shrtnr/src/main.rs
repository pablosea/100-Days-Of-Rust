
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

struct UrlShortener {
    urls: HashMap<String, String>,
    custom_urls: HashMap<String, String>,
    base_url: String,
}

impl UrlShortener {
    fn new(base_url: &str) -> Self {
        Self {
            urls: HashMap::new(),
            custom_urls: HashMap::new(),
            base_url: base_url.to_string(),
        }
    }

    fn shorten(&mut self, url: &str, custom: Option<&str>) -> Result<String, &'static str> {
        if !is_valid_url(url) { return Err("Invalid URL format"); }
        if url.len() > 200 { return Err("URL too long"); }

        let short_path = if let Some(custom_alias) = custom {
            if self.custom_urls.contains_key(custom_alias) {
                return Err("Custom alias already in use");
            }
            self.custom_urls.insert(custom_alias.to_string(), url.to_string());
            custom_alias.to_string()
        } else {
            let short = Self::generate_short_path();
            self.urls.insert(short.clone(), url.to_string());
            short
        };

        Ok(format!("{}/{}", self.base_url, short_path))
    }

    fn generate_short_path() -> String {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let random_part = now.wrapping_mul(1103515245).wrapping_add(12345);
        base62_encode(random_part)
    }
}

fn base62_encode(mut num: u128) -> String {
    const CHARSET: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    let mut result = Vec::new();
    while num > 0 {
        result.push(CHARSET[(num % 62) as usize]);
        num /= 62;
    }
    result.reverse();
    String::from_utf8(result).unwrap()
}

fn is_valid_url(url: &str) -> bool {
    url.starts_with("http://") || url.starts_with("https://")
}

fn create_shortener(base_url: &str) -> UrlShortener {
    UrlShortener::new(base_url)
}

fn shorten_url(shortener: &mut UrlShortener, url: &str, custom: Option<&str>) -> Result<String, &'static str> {
    shortener.shorten(url, custom)
}

fn main() {
    let mut shortener = create_shortener("https://short.url");
    match shorten_url(&mut shortener, "https://www.example.com", None) {
        Ok(short_url) => println!("Shortened URL: {}", short_url),
        Err(e) => println!("Error: {}", e),
    }
}



