use std::collections::{HashMap, HashSet};
use actix::prelude::{Actor, Context, Handler, Recipient};
use actix::dev::ToEnvelope;

use crate::structs::protocol::Protocol;
use crate::{errors, unwrap_clients_in_file};
use crate::structs::actors::relay::Relay;
use crate::structs::messages::{Connect, Disconnect, Error, Message, Response};
use crate::structs::internal::{Action, File};


/// A struct that mannage all the connections and client actors
pub struct Server<A = Relay>
where 
    A: Actor<Context = actix::Context<A>>,
    A: Protocol,
    A: Handler<Connect>,
    A: Handler<Message>,
    A: Handler<Disconnect>,
    A::Context: ToEnvelope<A, Connect>,
    A::Context: ToEnvelope<A, Message>,
    A::Context: ToEnvelope<A, Disconnect>
{
    /// The document actor factory.
    pub(in super) actor: fn(String, Recipient<Response>) -> A,
    /// A list of the opened files.
    pub(in super) files: HashMap<File, HashSet<u64>>,
    /// A list of the clients conected.
    pub(in super) clients: HashMap<u64, (Recipient<Message>, Recipient<Error>)>,
}


impl<A> Actor for Server<A> 
where
    A: Actor<Context = actix::Context<A>>,
    A: Protocol,
    A: Handler<Connect>,
    A: Handler<Message>,
    A: Handler<Disconnect>,
    A::Context: ToEnvelope<A, Connect>,
    A::Context: ToEnvelope<A, Message>,
    A::Context: ToEnvelope<A, Disconnect>,
{ type Context = Context<Self>; }

impl Server {
    fn none(name: String, server: Recipient<Response>) -> Relay { Relay::new(name, server) }
    
    /// Returns a server with the default document actor [`Relay`] 
    pub fn new() -> Server {
        Server { 
            actor: Self::none, 
            clients: HashMap::new(), 
            files: HashMap::new(), 
        }
    }
}

impl<A> Server<A>
where
    A: Actor<Context = actix::Context<A>>,
    A: Protocol,
    A: Handler<Connect>,
    A: Handler<Message>,
    A: Handler<Disconnect>,
    A::Context: ToEnvelope<A, Connect>,
    A::Context: ToEnvelope<A, Message>,
    A::Context: ToEnvelope<A, Disconnect>,
{
    /// Generates a Server with a factory for a custom document actor.
    pub fn new_with_actor(actor: fn(String, Recipient<Response>) -> A) -> Self
    {
        Self {
            actor: actor,
            files: HashMap::new(),
            clients: HashMap::new(),
        }
    }
    
    pub(super) fn send(&self, id: &u64, message: Message) {
        match self.clients.get(id) {
            Some((resipient, _)) => resipient.do_send(message),
            None => (),
        }
    }
    
    pub(super) fn send_err(&self, id: &u64, err: Error) {
        match self.clients.get(id) {
            Some((_, resipient)) => resipient.do_send(err),
            None => (),
        }
    }
    
    pub(super) fn get_clients_in_file(&self, key: &str) -> Result<(&File, &HashSet<u64>), Error> {
        match self.files.get_key_value(key) {
            Some(file) => Ok(file),
            None => Err( errors!(file_not_found key) ),
        }
    }
    
    pub(super) fn replicate(&self, msg: Message) {
        let (file, clients) = unwrap_clients_in_file!(self, msg => ());
        
        let message = Message {
            id: 0,
            file: file.name.clone(),
            mtype: msg.mtype,
            action: Action::None,
        };
        
        file.message.do_send(message.clone());
        clients.iter().for_each(
            |cl| {
                if *cl == msg.id { return }
                self.send(cl, message.clone());
            }
        );
    }
}
