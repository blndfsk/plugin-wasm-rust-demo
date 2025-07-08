use std::error::Error;

use http_wasm_guest::{
    host::{self, get_config, Request, Response},
    register, Guest,
};
use log::{error, info};
use regex::Regex;
use serde::Deserialize;

struct Plugin {
    pattern: Vec<Regex>,
}

impl Guest for Plugin {
    fn handle_request(&self, request: Request, response: Response) -> (bool, i32) {
        info!("req: {} {}", &request.method(), &request.uri());
        for regex in &self.pattern {
            if regex.is_match(request.uri().as_str()) {
                response.set_status(403);
                return (false, 0);
            }
        }
        (true, 0)
    }
}

#[derive(Deserialize)]
struct Config {
    pub rules: Vec<String>,
}

fn config() -> Result<Config, Box<dyn Error>> {
    let config: String = get_config()?;
    Ok(serde_json::from_str(&config)?)
}

fn main() {
    host::log::init().expect("no logging");
    let config: Config = config().expect("No valid config");
    info!("rules {:?}", &config.rules);

    let regex = config
        .rules
        .iter()
        .map(|string| Regex::new(string))
        .filter(|res| match res {
            Ok(_) => true,
            Err(err) => {
                error!("{}", err);
                false
            }
        })
        .map(|r| r.unwrap())
        .collect();
    let plugin = Plugin { pattern: regex };
    register(plugin);
}
