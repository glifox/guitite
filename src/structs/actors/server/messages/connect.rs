use std::collections::{HashMap, HashSet};
use actix::{ActorFutureExt, WrapFuture};
use actix::prelude::{Actor, Context, Handler, Recipient};
use actix::dev::{ContextFutureSpawner, ToEnvelope};

use super::super::Server;

use crate::errors;
use crate::structs::actors::relay::Relay;
use crate::structs::messages::{Connect, Disconnect, Error, Message, Mresult};
use crate::structs::internal::{Action, File, MessageType};


impl<A> Handler<Connect> for Server<A>
where 
    A: Actor<Context = actix::Context<A>>,
    A: Handler<Message>,
    A: Handler<Disconnect>,
    A::Context: ToEnvelope<A, Message>,
    A::Context: ToEnvelope<A, Disconnect>,
{
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Self::Context) -> Self::Result {
        let actor = (self.actor)(msg.file.clone()).start();
        
        let file = File { 
            name: msg.file.clone(),
            message: actor.clone().recipient(), 
            disconnect: actor.recipient() 
        };
        
        self.clients.insert(msg.id, (msg.addr_msg, msg.addr_err));
        
        match self.files.get_mut(&msg.file) {
            Some(val) =>{ val.insert(msg.id); },
            None => { 
                let mut set = HashSet::new();
                set.insert(msg.id);
                self.files.insert(file, set); 
            },
        };
    }
}