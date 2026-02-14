/// Litle hacky hack to work with the guitite-derive inside guitite crate.
mod guitite { pub use crate::*; }

use loro::LoroDoc;
use actix::{ActorContext, Handler, Recipient};
use guitite_derive::DocumentActor;

use crate::structs::protocol::Protocol;
use crate::structs::messages::{Disconnect, Response};


#[derive(Protocol, DocumentActor)]
#[document_actor(skip_disconnect)]
pub struct Relay {
    pub name: String,
    pub(in self) doc: LoroDoc,
    pub(in self) server: Recipient<Response>
}

impl Relay {
    pub(crate) fn new(name: String, server: Recipient<Response>) -> Self {
        log::debug!("Creating new");
        Self { name, doc: LoroDoc::new(), server}
    }
}

impl Handler<Disconnect> for Relay {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, ctx: &mut Self::Context) -> Self::Result {
        log::info!("Close message recived {:?}", msg);
        ctx.stop();
    }
}