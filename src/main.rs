use server::{Request, Response, Route, reqres::BodyContents};

use crate::server::Server;

pub mod server;

fn index(_req: &Request, res: &mut Response) {
    res.send_html("test_html/index.html");
}
fn test(req: &Request, res: &mut Response) {
    let json = req.body.as_ref().unwrap().as_json_object().unwrap();
    println!("{}", json.string("path").unwrap());
    println!("{}", json.i32("test_num").unwrap());
    println!("{}", json.object("test_obj").unwrap().string("name").unwrap());
    res.send_str("Thanks!");
}
fn test_arr(req: &Request, res: &mut Response) {
    let json = req.body.as_ref().unwrap().as_json_array().unwrap();
    dbg!(json);
    // println!("{}", json.string("path").unwrap());
    // println!("{}", json.i32("test_num").unwrap());
    // println!("{}", json.object("test_obj").unwrap().string("name").unwrap());
    res.send_str("Thanks!");
}
// fn custom_404(req: &Request, res: &mut Response) {
//     res.send_str("I can\'t find this page!!")
// }

fn main() {
    let root = Route::create("", server::RequestType::Get, index);
    let test = Route::create("/test", server::RequestType::Post, test);
    let test_arr = Route::create("/test_arr", server::RequestType::Post, test_arr);
    let mut server = Server::new(3000);
    server.register(root);
    server.register(test);
    server.register(test_arr);
    server.start();
}
