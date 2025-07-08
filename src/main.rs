use http_wasm_guest::{
    host::{self, Request, Response},
    register, Guest,
};
use log::{info, Level};
use regex::Regex;

mod config;

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

fn main() {
    host::log::init_with_level(Level::Debug).expect("no logging");
    let regex = config::read().expect("valid config");
    let plugin = Plugin { pattern: regex };
    register(plugin);
}
