use std::{
    io::{BufReader, Read},
    net::{TcpListener, TcpStream}
};

use super::{queue::RequestQueue, Request, RequestType, Response, reqres::BodyContents};

pub struct Server {
    routes: RouteStorage,
    listener: TcpListener,
    request_queue: RequestQueue,
}
impl Server {
    pub fn new(port: i32) -> Server {
        Server {
            routes: RouteStorage::new(),
            listener: TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap(),
            request_queue: RequestQueue::new(),
        }
    }

    pub fn register(&mut self, r: Route) {
        self.routes.add(r);
    }

    pub fn start(&mut self) {
        self.routes.prep();
        loop {
            if let Ok(conn) = self.listener.accept() {
                let (mut req_stream, _) = conn;
                let req_parsed = self.create_request_object(&mut req_stream);
                let mut matched_path: fn(&Request, &mut Response) = Server::default_error;
                if let Some(handler) = self
                    .routes
                    .handler(&req_parsed.request_type, &req_parsed.path)
                {
                    matched_path = handler;
                }

                let req = IncomingRequest {
                    request: req_parsed,
                    stream: req_stream,
                    route: matched_path,
                };
                self.request_queue.add(req);
            }
        }
    }

    fn create_request_object(&self, stream: &mut TcpStream) -> Request {
        let mut buffer = BufReader::new(stream);
        let mut headers_content = String::new();

        let mut cur_char: [u8; 1] = [0];
        let mut whitespace_count = 0;

        // Obtain headers
        loop {
            if let Ok(_) = buffer.read_exact(&mut cur_char) {
                let cur_char_val = char::from_u32(cur_char[0] as u32).unwrap();
                headers_content.push(cur_char_val);
                if cur_char_val == '\u{a}' || cur_char_val == '\u{d}' {
                    whitespace_count += 1;
                } else {
                    whitespace_count = 0;
                }
                // When we have a blank line, exit.
                if whitespace_count == 4 {
                    break;
                }
            } else {
                break;
            }
        }

        // Process headers
        let req: Vec<String> = headers_content
            .lines()
            .map(|a| a.to_string())
            .take_while(|a| !a.is_empty())
            .collect();
        let head = &req[0].split(" ").collect::<Vec<&str>>();

        let mut created_request = Request {
            path: head[1].to_string(),
            request_type: RequestType::type_for_method(head[0]),
            headers: req[1..]
                .to_vec()
                .iter()
                .map(|a| {
                    let d: Vec<&str> = a.split(": ").collect();
                    return (d[0].to_string(), d[1].to_string());
                })
                .collect(),
            body: None
        };

        if let Some(content_length_str) = created_request.headers.get("Content-Length") {
            // We have a body.
            let content_len: usize = content_length_str.parse().unwrap_or(0);
            let mut content: Vec<u8> = Vec::new();
            // Read body
            loop {
                if let Ok(_) = buffer.read_exact(&mut cur_char) {
                    content.push(cur_char[0]);
                    if content.len() >= content_len {
                        break;
                    }
                } else {
                    break;
                }
            }
            if let Some(content_type) = created_request.headers.get("Content-Type") {
                let no_charset = content_type.split(" ").collect::<Vec<&str>>()[0].replace(";", "");
                created_request.body = Some(BodyContents::type_from_mime(&no_charset, content));
            } else {
                created_request.body = Some(BodyContents::type_from_mime(&String::new(), content));
            }
        }
        return created_request;
    }

    fn default_error(_: &Request, res: &mut Response) {
        res.send_str("404 not found");
    }
}

pub struct Route {
    path: String,
    request_type: RequestType,
    handler: fn(&Request, &mut Response),
}
impl Route {
    pub fn create(
        path: &str,
        request_type: RequestType,
        handler: fn(&Request, &mut Response),
    ) -> Route {
        let mut resolved_path = String::new();
        if !path.starts_with("/") {
            resolved_path += "/";
        }
        resolved_path += path;
        Route {
            path: resolved_path,
            request_type,
            handler,
        }
    }
}

pub struct IncomingRequest {
    pub request: Request,
    pub stream: TcpStream,
    pub route: fn(&Request, &mut Response),
}

pub struct RouteStorage {
    routes_get: Vec<Route>,
    routes_post: Vec<Route>,
    routes_put: Vec<Route>,
    routes_delete: Vec<Route>,
    routes_any: Vec<Route>,
}

impl RouteStorage {
    fn new() -> RouteStorage {
        RouteStorage {
            routes_get: Vec::new(),
            routes_post: Vec::new(),
            routes_put: Vec::new(),
            routes_delete: Vec::new(),
            routes_any: Vec::new(),
        }
    }

    fn handler(
        &self,
        request_type: &RequestType,
        path: &String,
    ) -> Option<fn(&Request, &mut Response)> {
        let handler_cat = match request_type {
            RequestType::Get => &self.routes_get,
            RequestType::Post => &self.routes_post,
            RequestType::Put => &self.routes_put,
            RequestType::Delete => &self.routes_delete,
            RequestType::Any => &self.routes_any,
        };
        if let Ok(handler_ix) = handler_cat.binary_search_by(|a| a.path.cmp(path)) {
            Some(handler_cat[handler_ix].handler)
        } else if !request_type.is_any() {
            let any_ix = self
                .routes_any
                .binary_search_by(|a| a.path.cmp(path))
                .ok()?;
            Some(self.routes_any[any_ix].handler)
        } else {
            None
        }
    }
    fn add(&mut self, route: Route) {
        let handler_cat = match route.request_type {
            RequestType::Get => &mut self.routes_get,
            RequestType::Post => &mut self.routes_post,
            RequestType::Put => &mut self.routes_put,
            RequestType::Delete => &mut self.routes_delete,
            RequestType::Any => &mut self.routes_any,
        };
        handler_cat.push(route);
    }

    fn prep(&mut self) {
        self.routes_get.sort_by(|a, b| a.path.cmp(&b.path));
        self.routes_post.sort_by(|a, b| a.path.cmp(&b.path));
        self.routes_put.sort_by(|a, b| a.path.cmp(&b.path));
        self.routes_delete.sort_by(|a, b| a.path.cmp(&b.path));
        self.routes_any.sort_by(|a, b| a.path.cmp(&b.path));
    }
}
