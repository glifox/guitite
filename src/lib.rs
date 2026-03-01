//! # GUITITE
//! 
//! `Guitite` is a communication protocol desined to work with loro
//! this is the rust implementation for the protocol using actors, 
//! it is designed to allow custom actors to handle the messages.
//! 
//! ## Server
//! 
//! The `guitite::Server` is the orchestrator for the conections it 
//! keeps track of the opened __"files"__ and the clients conected to them.
//! 
//! ### Intanciation
//! The Server exposes 2 methods to _instanciate_ the server
//! 
//! 1. new: will create the simplest _server_. it contains a simple relay to demostrate sincronization with no persistance
//! 2. new_with_actor: this one allows to create a _custom actor_ that implements the Protocol, Actor, and the Handlers for _Connect_, _Message_, _Disconnect_.
//! 
//! ## Example
//! 
//! Create a simple relay server
//! ~~~no_run
//! use actix::{Actor, Addr};
//! use actix_web::{App, Error, HttpRequest, HttpResponse, HttpServer, get, web};
//! use actix_web_actors::ws;
//! use guitite::{Client, Server};
//! 
//! use env_logger;
//! 
//! #[get("/ws/{file_name}")]
//! async fn client(
//!     req: HttpRequest,
//!     stream: web::Payload,
//!     srv: web::Data<Addr<Server>>,
//!     path: web::Path<String>
//! ) -> Result<HttpResponse, Error> {
//!     log::info!("Try open {path}");
//!     ws::start( Client::new(path.as_str(), srv.get_ref().clone()), &req, stream, )
//! }
//! 
//! #[actix_web::main]
//! async fn main() -> std::io::Result<()> {
//!     env_logger::init();
//!     
//!     let server = Server::new().start();
//!     
//!     log::info!("starting HTTP server at http://localhost:8080");
//! 
//!     HttpServer::new(move || {
//!         App::new()
//!             .app_data(web::Data::new(server.clone()))
//!             .service(client)
//!     })
//!     .workers(2)
//!     .bind(("127.0.0.1", 8080))?
//!     .run()
//!     .await
//! }
//! ~~~ 
mod structs;
// mod aditionals;

pub use structs::actors::server::Server;
pub use structs::actors::client::Client;
pub use structs::protocol::Protocol;
pub use guitite_derive::DocumentActor;

#[allow(unused_imports)]
pub use structs::macros::*;

pub mod messages { pub use crate::structs::messages::*; }
pub mod types { 
    pub use crate::structs::internal::MessageType;
    pub use crate::structs::internal::Action;
}
pub mod actors {
    pub use crate::structs::actors::relay::Relay;
}
