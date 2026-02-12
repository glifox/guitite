use std::collections::HashSet;
use actix::{AsyncContext, Recipient};
use actix::prelude::{Actor, Handler};
use actix::dev::ToEnvelope;

use super::super::Server;

use crate::structs::messages::{Connect, Disconnect, Message, Response};
use crate::structs::internal::File;
use crate::structs::protocol::Protocol;


impl<A> Handler<Connect> for Server<A>
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
    type Result = ();

    fn handle(&mut self, msg: Connect, ctx: &mut Self::Context) -> Self::Result {
        let inbox: Recipient<Response> = ctx.address().recipient();
        let actor = (self.actor)(msg.file.clone(), inbox).start();
        
        let file = File { 
            name: msg.file.clone(),
            message: actor.clone().recipient(), 
            disconnect: actor.clone().recipient() 
        };
        
        let connect: Recipient<Connect> = actor.recipient();
        connect.do_send(msg.clone());
        
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