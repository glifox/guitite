# Guitite as a (Güitite _Acnistus arborescens_)

Its a protocol for loro-crdt, that abstrats the client-server 
comunication logic, it provides a tool set to allow you to create 
sync servers for the loro-crdt.

Guitite, is a custom designed protocol by @feraxhp in @glifox. Its aim 
is to provide a small, simple and fast library to sync various loro-crdt clients 

It is important to know that _guitite_, by _default_. does not mannage the loroDoc 
data persistance, it just mannage the comunication between peers.

> Güitite (Acnistus arborescens) is a fruit-bearing tree nicknamed "loro's delight" 
> because its sweet orange berries are a vital food source. (_loro is a parrot_).

## Server

The `guitite::Server` is the orchestrator for the conections it 
keeps track of the opened __"files"__ and the clients conected to them.

### Intanciation
The Server exposes 2 methods to _instanciate_ the server

1. new: will create the simplest _server_. it contains a simple relay to demostrate sincronization with no persistance
2. new_with_actor: this one allows to create a _custom actor_ that implements the Protocol, Actor, and the Handlers for _Connect_, _Message_, _Disconnect_.

### Example

Create a simple relay server
~~~rust
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
~~~ 