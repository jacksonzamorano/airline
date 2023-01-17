use airline_macros::ToJson;
use crate::{server::{Route, RequestType, Request, Response, Server, json::ToJson}, assets::Assets};

struct RequestConnector;
impl RequestConnector { 
    fn new() -> RequestConnector {
        RequestConnector {}
    }
}

fn index(_req: &Request, res: &mut Response, _data: &RequestConnector) {
    res.send_bytes(Assets::index());
}

pub fn create_demo() {
    let root = Route::create("", RequestType::Get, index);
    let mut server = Server::new(3000, RequestConnector::new);
    server.register(root);
    server.start();
}

#[derive(ToJson)]
pub struct Resource {
    pub id: String,
    pub friendly_name: String
}

#[derive(ToJson)]
pub struct LoginData {
    pub username: String,
    pub password: String,
    pub resources: Vec<Resource>, 
    pub is_superuser: bool
}