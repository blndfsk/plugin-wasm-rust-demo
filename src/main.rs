use http_wasm_guest::{
    host::{self, Request, Response},
    register, Guest,
};
use log::{error, info, Level};
use regex::Regex;

mod config;

struct Plugin {
    pattern: Vec<Regex>,
}

impl Guest for Plugin {
    fn handle_request(&self, request: Request, response: Response) -> (bool, i32) {
        info!("req: {} {}", &request.method(), &request.uri());
        info!("header: {:?}", &request.header().get());
        for regex in &self.pattern {
            match request.uri().to_str() {
                Ok(uri) => {
                    if regex.is_match(uri) {
                        response.set_status(403);
                        return (false, 0);
                    }
                }
                Err(err) => error!("{}", err.to_string()),
            }
        }
        (true, 0)
    }
}

fn main() {
    host::log::init_with_level(Level::Debug).expect("no logging");
    let regex = config::read().expect("no valid config");
    let plugin = Plugin { pattern: regex };
    register(plugin);
}
