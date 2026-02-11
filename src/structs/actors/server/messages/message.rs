use actix::prelude::{Actor, Handler};
use actix::dev::ToEnvelope;

use super::super::Server;

use crate::{errors, unwrap_clients_in_file};
use crate::structs::messages::{Connect, Disconnect, Message};
use crate::structs::internal::{Action, MessageType};

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
            ( // 11 - 41 (type - action)
                Action::Replicate, 
                MessageType::Export(_) | 
                MessageType::Ephimeral(_)
            ) => self.replicate(msg),
            (
                Action::Answer, 
                MessageType::VersionVector(_) |
                MessageType::None
            ) => file.message.do_send(msg),
            (a, m) => self.send_err(&msg.id, errors!( un_implemented a => m ))
        }
    }
}
