use std::collections::HashSet;

use actix::prelude::{Actor, Handler};
use actix::dev::ToEnvelope;

use super::super::Server;

use crate::structs::messages::{Connect, Disconnect, Message, Response, Target};


impl<A> Handler<Response> for Server<A>
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

    fn handle(&mut self, msg: Response, _: &mut Self::Context) -> Self::Result {
        let target: &HashSet<u64> = match &msg.target {
            Target::All(file) => {
                match self.files.get(file) {
                    Some(hs) => hs,
                    None => &HashSet::new(),
                }
            },
            Target::Client(client) => &HashSet::from([*client]),
        }; 
        log::debug!("Response: [{:?}]", msg);
        match msg.response {
            Ok(m) => {
                target.iter().for_each(|c| {
                    self.clients.get(c).map(|(r, _)| r.do_send(m.clone()));
                });
            },
            Err(e) => {
                target.iter().for_each(|c| {
                    self.clients.get(c).map(|(_, r)| r.do_send(e.clone()));
                });
            },
        };
    }
}