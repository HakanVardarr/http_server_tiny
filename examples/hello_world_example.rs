use http_server_tiny::{HttpServer, Method, Res};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = HttpServer::new("0.0.0.0:8000", "./static/error.html");
    server.add_route(
        &Method::Get,
        "/",
        Box::new(|_| Res::File {
            name: "./static/index.html",
            ct: "text/html; charset=utf-8",
            sc: 200,
        }),
    );

    server.handle_requests(Box::new(|req| {
        println!("INFO: {} {}", req.method, req.url);
    }))
}
