use std::collections::{HashMap, HashSet};
use actix::{ActorFutureExt, WrapFuture};
use actix::prelude::{Actor, Context, Handler, Recipient};
use actix::dev::{ContextFutureSpawner, ToEnvelope};

use super::super::Server;

use crate::errors;
use crate::structs::actors::relay::Relay;
use crate::structs::messages::{Connect, Disconnect, Error, Message, Mresult};
use crate::structs::internal::{Action, File, MessageType};

impl<A> Handler<Message> for Server<A>
where 
    A: Actor<Context = actix::Context<A>>,
    A: Handler<Message>,
    A: Handler<Disconnect>,
    A::Context: ToEnvelope<A, Message>,
    A::Context: ToEnvelope<A, Disconnect>,
{
    type Result = Mresult;

    fn handle(&mut self, msg: Message, _: &mut Self::Context) -> Self::Result {
        match (&msg.action, &msg.mtype) {
            ( // 11 - 41 (type - action)
                Action::Replicate, 
                MessageType::Export(_) | 
                MessageType::Ephimeral(_)
            ) => self.replicate(msg),
            (
                Action::Answer, 
                MessageType::VersionVector(_) |
                MessageType::None
            ) =>{ let _ = self.respond_version(msg); },
            (a, m) => self.send_err(&msg.id, errors!( un_implemented a => m ))
        };
        
        None
    }
}
