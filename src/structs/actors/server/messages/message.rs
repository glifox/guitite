use actix::prelude::{Actor, Handler};
use actix::dev::ToEnvelope;

use super::super::Server;

use crate::unwrap_clients_in_file;
use crate::structs::messages::{Connect, Disconnect, Message};
use crate::structs::internal::{Action};

impl<A> Handler<Message> for Server<A>
where 
    A: Actor<Context = actix::Context<A>>,
    A: Handler<Connect>,
    A: Handler<Message>,
    A: Handler<Disconnect>,
    A::Context: ToEnvelope<A, Connect>,
    A::Context: ToEnvelope<A, Message>,
    A::Context: ToEnvelope<A, Disconnect>,
{
    type Result = ();

    fn handle(&mut self, msg: Message, _: &mut Self::Context) -> Self::Result {
        let (file, _) = unwrap_clients_in_file!(self, msg);
        
        match (&msg.action, &msg.mtype) {
            ( Action::Replicate, _ ) => self.replicate(msg),
            ( Action::Answer | Action::None, _) => file.message.do_send(msg),
        }
    }
}
