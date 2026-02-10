use std::collections::{HashMap, HashSet};
use actix::{ActorFutureExt, WrapFuture};
use actix::prelude::{Actor, Context, Handler, Recipient};
use actix::dev::{ContextFutureSpawner, ToEnvelope};

use crate::{errors, unwrap_clients_in_file};
use crate::structs::actors::relay::Relay;
use crate::structs::messages::{Connect, Disconnect, Error, Message, Mresult};
use crate::structs::internal::{Action, File, MessageType};

use super::super::Server;

impl<A> Handler<Disconnect> for Server<A>
where 
    A: Actor<Context = actix::Context<A>>,
    A: Handler<Message>,
    A: Handler<Disconnect>,
    A::Context: ToEnvelope<A, Message>,
    A::Context: ToEnvelope<A, Disconnect>,
{
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Self::Context) -> Self::Result {
        self.clients.remove(&msg.id);
        
        match self.files.get_mut(&msg.file) {
            Some(v) => v.remove(&msg.id),
            None => return ,
        };
        
        let (file, clients ) = unwrap_clients_in_file!(self, msg);
        if clients.is_empty() { file.disconnect.do_send(msg.clone()); }
    }
}