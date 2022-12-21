# Airline
No-dependancy multithreaded web server.

**This is not ready for production yet!**

## About Airline
Airline is intended to simplify a lot of the additional complexity added by other packages. Instead of requiring a ton of packages, and in some cases a runtime, Airline is pure Rust with no dependancies.

## Overview & Usage
```
struct RequestConnector;
impl RequestConnector { 
    fn new() -> RequestConnector {
        RequestConnector {}
    }
}

fn index(_req: &Request, res: &mut Response, _data: &RequestConnector) {
    res.send_str(Assets::INDEX);
}

pub fn main() {
    let root = Route::create("", RequestType::Get, index);
    let mut server = Server::new(3000, RequestConnector::new);
    server.register(root);
    server.start();
}
```

For each route you want to register, create a function that accepts a borrow of a `Request`, a mutable borrow of `Response`, and a borrow of any class that is `
'static + Send`. The `Request` contains info about the request, and the `Response` is a struct that this function should update in order to actually return data to the client. Finally, the last argument is a struct type that you provide. When creating the server, you pass a function that creates an instance of this struct. One struct is created per thread, and each thread has full access to the struct. This is a great place to include required components, such as database connections or any API connectors.

Then, create a route struct. It accepts a path argument as String (trailing/leading slash not required), a RequestType (a HTTP verb), and the function you created.

Next, create the server. Pass a port number, and a function that creates whichever struct you want to be passed to your functons.

`Register` all your routes, call `start` and your server should be all good to go!

## Asset Compilation

If you are serving `HTML` files, you can build these into your binary. Install this crate as an executable with `cargo install`, and then call `airline compile path_to_html` from within your project root. All of your HTML files will be extracted and copied to `const &str`s in a struct called `Assets`, stored within `src/assets.rs`.

Note you still can send HTML files read straight from the file system. This is done by calling `response.send_html(path)`.