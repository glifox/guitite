use actix::{Actor, Context, Handler};

use crate::structs::messages::{Disconnect, Message, Mresult};

pub struct Relay(pub String);

impl Actor for Relay { type Context = Context<Self>; }

impl Handler<Message> for Relay {
    type Result = Mresult;

    fn handle(&mut self, msg: Message, _ctx: &mut Self::Context) -> Self::Result {
        log::info!("Message recived: {:#?}", msg);
        
        Some(Ok(msg))
    }
}

impl Handler<Disconnect> for Relay {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _ctx: &mut Self::Context) -> Self::Result {
        log::info!("Close message recived {:?}", msg)
    }
}