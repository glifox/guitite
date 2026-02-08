use std::collections::{HashMap, HashSet};
use actix::{ActorFutureExt, WrapFuture};
use actix::prelude::{Actor, Context, Handler, Recipient};
use actix::dev::{ContextFutureSpawner, ToEnvelope};

use crate::errors;
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
        match self.files.get_mut(&msg.file) {
            Some(val) => val.remove(&msg.id),
            None => unreachable!(),
        };
        
        match self.files.get_key_value(&msg.file) {
            Some((k, v)) => {
                if let Some(dis) = &k.disconnect && v.is_empty() {
                    dis.do_send(Disconnect { id: msg.id, file: msg.file.clone() });
                }
            },
            None => unreachable!(),
        };
        
        self.clients.remove(&msg.id);
    }
}