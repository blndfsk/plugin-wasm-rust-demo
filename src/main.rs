use http_wasm_guest::{host::get_config, register, request::Request, response::Response, Guest};
use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    pub rules: Vec<String>,
}

struct Plugin {}

impl Guest for Plugin {
    fn handle_request(&self, _request: Request, _response: Response) -> bool {
        true
    }

    fn handle_response(&self, _request: Request, _response: Response) {}
}
fn main() {
    let _config: Option<Config> =
        get_config().and_then(|string| serde_json::from_slice(&string).ok());
    let plugin = Plugin {};

    register(plugin);
}
