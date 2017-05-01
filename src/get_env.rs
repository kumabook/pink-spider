use std::env;
use toml::Value;
use std::fs::File;
use std::io::Read;

pub fn var(key: &str) -> Option<String> {
    match env::var(key) {
        Ok(value) => Some(value),
        Err(_) => {
            let file = File::open("config/env.toml");
            if file.is_err() {
                return None;
            }
            let mut f = file.unwrap();
            let mut s = String::new();
            let _ = f.read_to_string(&mut s);
            if let Ok(value) = s.parse::<Value>() {
                value.as_table()
                    .and_then(|t| t.get(key))
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string())
            } else {
                None
            }
        }
    }
}
