
use actix::{Actor, Addr};
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Error, web};
use actix_web_actors::ws;
use guitite::{Client, Server};

use env_logger;

async fn client(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<Server>>,
) -> Result<HttpResponse, Error> {
    log::info!("Opened recived");
    ws::start( Client::new("file", srv.get_ref().clone()), &req, stream, )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    
    let server = Server::new().start();
    
    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(server.clone()))
            .route("/ws", web::get().to(client))
            
    })
    .workers(2)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}