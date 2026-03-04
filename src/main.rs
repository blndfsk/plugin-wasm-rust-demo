use http_wasm_guest::{
    Guest,
    host::{self, Request, Response},
    register,
};
use log::{Level, info};

struct Plugin {}

impl Guest for Plugin {
    fn handle_request(&self, request: &Request, _response: &Response) -> (bool, i32) {
        info!("URI: {}", request.uri());
        request.header().add(b"X-Foo", b"bar");
        (true, 0)
    }
}

fn main() {
    let _ = host::admin::init_log_with_level(Level::Debug);
    let plugin = Plugin {};
    register(plugin);
}
