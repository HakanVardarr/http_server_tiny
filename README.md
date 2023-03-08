# http_server_tiny
Tiny http server library using tiny http

# Usage: 
```rust
use http_server_tiny::{HttpServer, Method, Res};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = HttpServer::new("0.0.0.0:9975", "./error.html");
    server.add_route(
        &Method::Get,
        "/",
        Box::new(|_| Res::File {
            name: "./index.html",
            ct: "text/html; charset=utf-8",
            sc: 200,
        }),
    );
    server.add_route(
        &Method::Get,
        "/index.js",
        Box::new(|_| Res::File {
            name: "./index.js",
            ct: "text/javascript; charset=utf-8",
            sc: 200,
        }),
    );
    server.add_route(
        &Method::Get,
        "/api/name",
        Box::new(|_req| Res::Json(r"{'name': 'Hakan'}".to_string())),
    );

    server.add_route(
        &Method::Post,
        "/api/search",
        Box::new(|req| {
            println!("{}", req.content);
            Res::Empty
        }),
    );

    server.handle_requests()
}


```
