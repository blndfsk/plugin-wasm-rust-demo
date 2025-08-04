use std::{borrow::Cow, error::Error, str::FromStr};

use http_wasm_guest::host;

use regex::Regex;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Config<'a> {
    #[serde(borrow)]
    rules: Vec<Cow<'a, str>>,
}

pub(crate) fn read() -> Result<Vec<Regex>, Box<dyn Error>> {
    let bytes = host::config();
    to_regexp(&bytes)
}

fn to_regexp(bytes: &[u8]) -> Result<Vec<Regex>, Box<dyn Error>> {
    let config: Config = serde_json::from_slice(bytes)?;
    let result: Vec<Regex> = config
        .rules
        .iter()
        .map(|raw| Regex::from_str(&raw))
        .filter(|res| res.is_ok())
        .map(|r| r.unwrap())
        .collect();
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        let json = br#"{ "rules" : ["^/\\.config"]}"#;
        let c = to_regexp(json).unwrap();
        assert!(c.get(0).unwrap().is_match(r"/.config"));
    }
}
