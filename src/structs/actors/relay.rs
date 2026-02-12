use std::borrow::Cow;
use actix::{Actor, ActorContext, Context, Handler, Recipient};
use loro::{ExportMode, LoroDoc, VersionVector};

use crate::structs::conversor::Conversor;
use crate::{errors, message};
use crate::structs::messages::{Connect, Disconnect, Error, Message, Response, Target};
use crate::structs::internal::{Action, MessageType};


pub struct Relay {
    pub name: String,
    pub(in self) doc: LoroDoc,
    pub(in self) server: Recipient<Response>
}

impl Relay {
    pub(crate) fn new(name: String, server: Recipient<Response>) -> Self {
        Self { name, doc: LoroDoc::new(), server}
    }
    
    fn version_vector(&self) -> Vec<u8> { self.doc.state_vv().encode() }
    
    fn export_updates(&self, vv: &[u8]) -> Result<Vec<u8>, Error> {
        let version: VersionVector = VersionVector::decode(vv)
            .map_err(|e| e.to_error())?;
        
        self.doc.export(ExportMode::Updates { from: Cow::Borrowed(&version) })
            .map_err(|e| e.to_error())
    }
    
    fn import(&self, bytes: &[u8]) -> Result<bool, Error> {
        let status = self.doc.import(bytes)
            .map_err(|e| e.to_error())?;
        
        Ok(status.pending.is_none())
    }
}

impl Actor for Relay { type Context = Context<Self>; }

impl Handler<Connect> for Relay {
    type Result = ();
    
    fn handle(&mut self, msg: Connect, _: &mut Self::Context) -> Self::Result {        
        msg.addr_msg.do_send(Message {
            id: msg.id,
            file: msg.file,
            mtype: MessageType::None,
            action: Action::Answer,
        });
    }
}

impl Handler<Message> for Relay {
    type Result = ();

    fn handle(&mut self, msg: Message, _ctx: &mut Self::Context) -> Self::Result {
        log::debug!("[>] retrive: {:?}", msg);
        macro_rules! response {
            () => { todo!() };
            (   $response:expr => $target:expr) => { self.server.do_send(Response { target: $target, response: $response }) };
            (er $response:expr => $target:expr) => { self.server.do_send(Response { target: $target, response: Err( $response ) }) };
            (ok $response:expr => $target:expr) => { self.server.do_send(Response { target: $target, response: Ok ( $response ) }) };
        }

        match (&msg.mtype, &msg.action) {
            (MessageType::VersionVector(version), Action::Answer)
            => {
                let update = self.export_updates(version);
                let update = update.map(|c| {
                    message!(copy msg, MessageType::Export(c), Action::None)
                });
                response!(update => Target::Client(msg.id))
            },
            (MessageType::Export(bytes), Action::None)
            => {
                match self.import(bytes) {
                    Ok(true) => (),
                    Ok(false) => {
                        let mtype = MessageType::VersionVector(self.version_vector());
                        response!(ok message!(copy msg, mtype, Action::Answer) => Target::Client(msg.id))
                    },
                    Err(e) => response!(er e => Target::Client(msg.id)),
                }
            },
            (a, m) 
            => response!(er errors!( un_implemented a => m ) => Target::Client(msg.id))
        };
    }
}

impl Handler<Disconnect> for Relay {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, ctx: &mut Self::Context) -> Self::Result {
        log::info!("Close message recived {:?}", msg);
        log::info!("Doc text state: {:?}", self.doc.get_text("text").to_string());
        ctx.stop();
    }
}