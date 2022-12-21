use server::{Request, Response, Route};
use crate::server::Server;

pub mod server;

struct TestDatabase {
    database_connected: bool
}
impl TestDatabase {
    fn create() -> TestDatabase {
        TestDatabase { database_connected: true }
    }
    pub fn request(&self, query: TestDatabaseQuery) -> TestDatabaseRecord {
        return TestDatabaseRecord { from_query: query.query };
    }
}
struct TestDatabaseQuery {
    query: String
}
#[derive(Debug)]
struct TestDatabaseRecord {
    from_query: String
}
fn index(_req: &Request, res: &mut Response, data: &TestDatabase) {
    dbg!(data.request(TestDatabaseQuery { query: "select test query".to_string() }));
    res.send_html("test_html/index.html");
}
fn test(req: &Request, res: &mut Response, data: &TestDatabase) {
    let json = req.body.as_ref().unwrap().as_json_object().unwrap();
    let table = json.string("type").unwrap();
    let filters = json.array("filters").unwrap();

    let mut query = String::new();
    query += "select * from ";
    query += &table;
    if filters.all().len() > 0 {
        query += " where ";
        query += &filters.all().iter().map(|a| {
            let b = a.object().unwrap();
            let k = b.string("key").unwrap();
            let v = b.string("value").unwrap();
            let mut output = String::new();
            output += &k;
            output += " = ";
            output += &v;
            return output;
        }).collect::<Vec<String>>().join(" ");
    }
    query += ";";
    res.send_str(&format!("{}", data.request(TestDatabaseQuery { query }).from_query));
}
fn test_arr(req: &Request, res: &mut Response, data: &TestDatabase) {
    let json = req.body.as_ref().unwrap().as_json_array().unwrap();
    dbg!(json);
    // println!("{}", json.string("path").unwrap());
    // println!("{}", json.i32("test_num").unwrap());
    // println!("{}", json.object("test_obj").unwrap().string("name").unwrap());
    res.send_str("Thanks!");
}
fn create_env() -> TestDatabase {
    TestDatabase::create()
}

fn main() {
    let root = Route::create("", server::RequestType::Get, index);
    let test = Route::create("/test", server::RequestType::Post, test);
    let test_arr = Route::create("/test_arr", server::RequestType::Post, test_arr);
    let mut server = Server::new(3000, create_env);
    server.register(root);
    server.register(test);
    server.register(test_arr);
    server.start();
}
