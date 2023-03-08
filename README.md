# http_server_tiny
Tiny http server library using tiny http

# Usage: 


You need to provide a error.html file to use when server return 404 error.


## src/main.rs

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
        Box::new(|_| Res::Json(r"{'name': 'Hakan'}".to_string())),
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
## index.html
```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta http-equiv="X-UA-Compatible" content="IE=edge" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Hello, World</title>
  </head>
  <body>
    <h1>Hello, world!</h1>
  </body>
  <script src="index.js"></script>
</html>
```
## index.js
```javascript
fetch("/api/search", {
    method: 'POST',
    body: 'Hakan',
})

```
## error.html
```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta http-equiv="X-UA-Compatible" content="IE=edge" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>404</title>
  </head>
  <body>
    404
  </body>
</html>

```
