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
mod structs;

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
