use crate::{server::{Route, RequestType, Request, Response, Server}, assets::Assets};

struct RequestConnector;
impl RequestConnector { 
    fn new() -> RequestConnector {
        RequestConnector {}
    }
}

fn index(_req: &Request, res: &mut Response, _data: &RequestConnector) {
    res.send_str(Assets::INDEX);
}

pub fn create_demo() {
    let root = Route::create("", RequestType::Get, index);
    let mut server = Server::new(3000, RequestConnector::new);
    server.register(root);
    server.start();
}