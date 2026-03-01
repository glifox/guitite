
use actix::{Actor, Addr};
use actix_web::{App, Error, HttpRequest, HttpResponse, HttpServer, get, web};
use actix_web_actors::ws;
use guitite::{Client, Server};

use env_logger;

#[get("/ws/{file_name}")]
async fn client(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<Server>>,
    path: web::Path<String>
) -> Result<HttpResponse, Error> {
    log::info!("Try open {path}");
    ws::start( Client::new(path.as_str(), srv.get_ref().clone()), &req, stream, )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    
    let server = Server::new().start();
    
    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(server.clone()))
            .service(client)
    })
    .workers(2)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
