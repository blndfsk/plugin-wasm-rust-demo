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
        match request.uri() {
            Some(s) if s.starts_with("/.config".as_bytes()) => {
                response.set_status_code(403);
                return (false, 0);
            }
            _ => {}
        }
        (true, 0)
    }
}
fn main() {
    let config: Config = get_config()
        .and_then(|v| serde_json::from_slice(&v).ok())
        .unwrap();
    info!("rules {:?}", &config.rules);

    let plugin = Plugin {};
    register(plugin);
}
