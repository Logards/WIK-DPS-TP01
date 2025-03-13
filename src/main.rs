use actix_web::{App, HttpServer, web, HttpRequest, HttpResponse};
use std::env;
use std::collections::HashMap;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = match env::var("PING_LISTEN_PORT") {
        Ok(port) => match port.parse::<u16>() {
            Ok(port_num) => port_num,
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        },
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };
    println!("Starting server on port {}", port);
    match HttpServer::new(|| App::new().route("/ping", web::get().to(ping), ).default_service(web::route().to(not_found)))
        .bind(("127.0.0.1", port))
    {
        Ok(server) => server.run().await,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

async fn ping(req: HttpRequest) -> HttpResponse {
    let headers: HashMap<String, String> = req.headers()
        .iter()
        .map(|(name, value)| {
            (
                name.to_string(),
                value.to_str().unwrap_or("Non-UTF-8 value").to_string()
            )
        })
        .collect();

    HttpResponse::Ok()
        .content_type("application/json")
        .json(headers)
}

async fn not_found() -> HttpResponse {
    HttpResponse::NotFound().finish()
}
