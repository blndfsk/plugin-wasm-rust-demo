use http_wasm_guest::{
    host::get_config, info, register, request::Request, response::Response, Guest,
};
use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    pub rules: Vec<String>,
}

struct Plugin {}

impl Guest for Plugin {
    fn handle_request(&self, request: Request, _response: Response) -> (bool, i32) {
        info!("uri: {}", to_str(request.uri().as_deref()));
        (true, 0)
    }

    fn handle_response(&self, _request: Request, _response: Response) {}
}
fn main() {
    let config: Config = get_config()
        .and_then(|string| serde_json::from_slice(&string).ok())
        .unwrap();
    info!("rules {:?}", &config.rules);

    let plugin = Plugin {};

    register(plugin);
}

fn to_str(vec: Option<&[u8]>) -> String {
    vec.and_then(|v| std::str::from_utf8(v).ok())
        .unwrap_or_default()
        .to_string()
}
