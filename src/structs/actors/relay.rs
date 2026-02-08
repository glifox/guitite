use actix::{Actor, Context, Handler};

use crate::structs::messages::{Disconnect, Message, Mresult};

pub struct Relay;

impl Actor for Relay { type Context = Context<Self>; }

impl Handler<Message> for Relay {
    type Result = Mresult;

    fn handle(&mut self, _msg: Message, _ctx: &mut Self::Context) -> Self::Result {
        unreachable!()
    }
}

impl Handler<Disconnect> for Relay {
    type Result = ();

    fn handle(&mut self, _msg: Disconnect, _ctx: &mut Self::Context) -> Self::Result {
        unreachable!()
    }
}