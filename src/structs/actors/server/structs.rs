use std::collections::{HashMap, HashSet};
use actix::prelude::{Actor, Context, Handler, Recipient};
use actix::dev::ToEnvelope;

use actix::dev::ContextFutureSpawner;
use actix::{ActorFutureExt, WrapFuture};

use crate::{errors, unwrap_clients_in_file};
use crate::structs::actors::relay::Relay;
use crate::structs::messages::{Disconnect, Error, Message};
use crate::structs::internal::{Action, File, MessageType};

pub struct Server<A = Relay>
where 
    A: Actor<Context = actix::Context<A>>,
    A: Handler<Message>,
    A: Handler<Disconnect>,
    A::Context: ToEnvelope<A, Message>,
    A::Context: ToEnvelope<A, Disconnect>
{
    pub(in super) actor: fn(String) -> A,
    pub(in super) files: HashMap<File, HashSet<u64>>,
    pub(in super) clients: HashMap<u64, (Recipient<Message>, Recipient<Error>)>,
}


impl<A> Actor for Server<A> 
where
    A: Actor<Context = actix::Context<A>>,
    A: Handler<Message>,
    A: Handler<Disconnect>,
    A::Context: ToEnvelope<A, Message>,
    A::Context: ToEnvelope<A, Disconnect>,
{ type Context = Context<Self>; }

impl Server {
    #[allow(unused)]
    fn none(name: String) -> Relay { Relay(name) }
    
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
    A: Handler<Message>,
    A: Handler<Disconnect>,
    A::Context: ToEnvelope<A, Message>,
    A::Context: ToEnvelope<A, Disconnect>,
{
    pub fn new_with_actor(actor: fn(String) -> A) -> Self
    {
        Self {
            actor: actor,
            files: HashMap::new(),
            clients: HashMap::new(),
        }
    }
    
    pub(super) fn send(&self, id: &u64, message: Message) {
        match self.clients.get(id) {
            Some((resipient, _)) => resipient.do_send(message.clone()),
            None => (),
        }
    }
    
    pub(super) fn send_err(&self, id: &u64, err: Error) {
        match self.clients.get(id) {
            Some((_, resipient)) => resipient.do_send(err.clone()),
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
    
    pub(super) fn respond_version(&self, msg: Message, ctx: &mut Context<Server<A>>) {
        let (file, _) = unwrap_clients_in_file!(self, msg);
        
        let id = msg.id.clone();
        let is_new = matches!((&msg.action, &msg.mtype),(Action::None, MessageType::None));
        
        file.message.send(msg)
            .into_actor(self)
            .then(
                move |response, server , _| {
                    log::info!("responded");
                    
                    match response {
                        Ok(Some(Ok(m))) => {
                            match (&m.mtype, &m.action) {
                                (MessageType::Export(_), Action::None) |
                                (MessageType::None, Action::None) if !is_new => server.send(&id, m),
                                (MessageType::None, Action::None) if is_new => server.send_err(&id, errors!(file_not_found)),
                                _ => panic!("The response with a version must be type: (Export | None) and action (None)"),
                            }
                        },
                        Ok(Some(Err(e))) => server.send_err(&id, e),
                        Ok(None) => panic!("The file actor must return the export for the version"),
                        Err(e) => {
                            let err = Error { status: 500, message: e.to_string(), fatal: true };
                            server.send_err(&id, err);
                        },
                    }
                    
                    actix::fut::ready(())
                }
            ).wait(ctx);
    }
}
