use actix::prelude::{Actor, Handler};
use actix::dev::ToEnvelope;

use crate::structs::protocol::Protocol;
use crate::unwrap_clients_in_file;
use crate::structs::messages::{Connect, Disconnect, Message};

use super::super::Server;

impl<A> Handler<Disconnect> for Server<A>
where 
    A: Actor<Context = actix::Context<A>>,
    A: Protocol,
    A: Handler<Connect>,
    A: Handler<Message>,
    A: Handler<Disconnect>,
    A::Context: ToEnvelope<A, Connect>,
    A::Context: ToEnvelope<A, Message>,
    A::Context: ToEnvelope<A, Disconnect>,
{
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Self::Context) -> Self::Result {
        log::debug!("disconect recived: {} on {}", msg.id, msg.file);
        match self.clients.remove(&msg.id){
            Some(_) => (),
            None => log::error!("can not remove client"),
        };
        
        match self.files.get_mut(&msg.file) {
            Some(v) => v.remove(&msg.id),
            None => {log::error!("Unexpencted emptyness"); false},
        };
        
        let (file, clients ) = unwrap_clients_in_file!(self, msg);
        if clients.is_empty() { 
            file.disconnect.do_send(msg.clone());
            self.files.remove(&msg.file);
        };
        
        log::debug!("state: \nfiles: {:#?}\nclients: {:#?}", self.files, self.clients);
    }
}