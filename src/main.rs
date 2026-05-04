use http_wasm_guest::{
    Guest, HostLogger,
    host::{Request, Response},
    register,
};

struct Plugin {}

impl Guest for Plugin {
    fn handle_request(&self, request: &Request, _response: &Response) -> (bool, i32) {
        log::info!("URI: {}", request.uri());
        request.header.add(b"X-Foo", b"bar");
        (true, 0)
    }
}

fn main() {
    let _ = HostLogger::init();
    let plugin = Plugin {};
    register(plugin);
}
