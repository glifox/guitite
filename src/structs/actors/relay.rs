use loro::LoroDoc;
use actix::{Actor, ActorContext, Context, Handler, Recipient};

use crate::structs::protocol::Protocol;
use crate::message;
use crate::structs::messages::{Connect, Disconnect, Message, Response, Target};
use crate::structs::internal::{Action, MessageType};


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
    
    fn do_send(&self, msg: Response) { 
        log::debug!("[>]  sended: {:?}", msg);
        self.server.do_send(msg);
    }
}

impl Protocol for Relay { }
impl Actor for Relay { type Context = Context<Self>; }

impl Handler<Connect> for Relay {
    type Result = ();
    
    fn handle(&mut self, msg: Connect, _: &mut Self::Context) -> Self::Result { 
        let version = self.doc.state_vv().encode();
        msg.addr_msg.do_send(message!(copy msg, MessageType::VersionVector(version), Action::Answer));
    }
}

impl Handler<Message> for Relay {
    type Result = ();

    fn handle(&mut self, msg: Message, _ctx: &mut Self::Context) -> Self::Result {
        log::debug!("[<] recived: {:?}", msg);
        let id = msg.id.clone();
        let response = self.process(&self.doc, msg);
        
        match response {
            Ok(Some(m)) => self.do_send(Response { target: Target::Client(id), response: Ok(m) }),
            Err(e) => self.do_send(Response { target: Target::Client(id), response: Err(e) }),
            Ok(None) => (),
        };
    }
}

impl Handler<Disconnect> for Relay {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, ctx: &mut Self::Context) -> Self::Result {
        log::info!("Close message recived {:?}", msg);
        log::info!("Doc text state: {:?}", self.doc.get_text("codemirror").to_string());
        ctx.stop();
    }
}