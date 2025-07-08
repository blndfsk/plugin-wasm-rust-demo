use std::{borrow::Cow, error::Error, str::FromStr};

use http_wasm_guest::host::get_config;

use log::debug;
use regex::Regex;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Config<'a> {
    #[serde(borrow)]
    rules: Vec<Cow<'a, str>>,
}

fn config() -> Result<String, Box<dyn Error>> {
    let s = get_config()?;
    debug!("config: {}", &s);
    Ok(s)
}

fn to_regexp(str: &str) -> Result<Vec<Regex>, Box<dyn Error>> {
    let config: Config = serde_json::from_str(str)?;
    let result: Vec<Regex> = config
        .rules
        .iter()
        .map(|raw| Regex::from_str(&raw))
        .filter(|res| res.is_ok())
        .map(|r| r.unwrap())
        .collect();
    Ok(result)
}

pub(crate) fn read() -> Result<Vec<Regex>, Box<dyn Error>> {
    to_regexp(&config()?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        let json = r#"{ "rules" : ["^/\\.config"]}"#;
        let vec = to_regexp(&json).unwrap_or_else(|err| panic!("parse error: {}", err));
        for r in &vec {
            assert!(r.is_match(r#"/.config"#), "regex: {}, ", r);
        }
    }
    #[test]
    fn test_regexp() {
        let regex = Regex::new(r"^/\.config").unwrap();
        assert!(regex.is_match(r"/.config"));
    }
}
