use std::error::Error;

use http_wasm_guest::{
    host::{get_config, Request, Response},
    info, register, Guest,
};
use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    pub rules: Vec<String>,
}

struct Plugin {}

impl Guest for Plugin {
    fn handle_request(&self, request: Request, response: Response) -> (bool, i32) {
        if request.uri().starts_with(b"/.config") {
            response.set_status(403);
            return (false, 0);
        }
        (true, 0)
    }
}
fn config() -> Result<Config, Box<dyn Error>> {
    let c = get_config()?;
    Ok(serde_json::from_str(&c)?)
}
fn main() {
    let config: Config = config().expect("No valid config");
    info!("rules {:?}", &config.rules);

    let plugin = Plugin {};
    register(plugin);
}
