use std::collections::{HashMap, HashSet};
use actix::{ActorFutureExt, WrapFuture};
use actix::prelude::{Actor, Context, Handler, Recipient};
use actix::dev::{ContextFutureSpawner, ToEnvelope};

use super::super::Server;

use crate::{errors, unwrap_clients_in_file, wait};
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

    fn handle(&mut self, msg: Message, ctx: &mut Self::Context) -> Self::Result {
        match (&msg.action, &msg.mtype) {
            (
                Action::Replicate, 
                MessageType::Export(_) | 
                MessageType::Ephimeral(_)
            ) => self.replicate(msg),
            (
                Action::Answer, 
                MessageType::VersionVector(_)
            ) => {
                let (file, clients) = unwrap_clients_in_file!(self, msg);
    
                if let Some(res) = &file.message { 
                    res.do_send(msg); // It must send a "export - passthrout" back to respond to the client
                    return None
                }
                // If no local actor, send the request to a random client previusly connected
                let id = match clients.iter().find(|u| **u != msg.sender_id) {
                    Some(id) => id,
                    None => {
                        self.send_err(&msg.sender_id, errors!(no_clients));
                        return None
                    },
                };
                self.send(id, msg); // It must respond a "export - replicate" to responde the changes (or None - None)
            }
            (
                Action::Answer,
                MessageType::None
            ) => {
                let (file, _) = unwrap_clients_in_file!(self, msg);
                
                if let Some(res) = &file.message {
                    
                    let id = msg.sender_id.clone();
                    
                    res.send(msg).into_actor(self)
                        .then(
                            move |res, server, _| { 
                                match res {
                                    Ok(Some(Ok(m))) => server.send(&id, m),
                                    Ok(Some(Err(e))) => server.send_err(&id, e),
                                    Ok(None) => panic!("The file actor must return the export for the version"),
                                    Err(e) => {
                                        let err = Error { status: 500, message: e.to_string(), fatal: true };
                                        server.send_err(&id, err);
                                    },
                                }
                                actix::fut::ready(()) 
                            }
                        ).wait(ctx); // wait until the future resolves
                    return None
                }
                
                
                
            }
            (a, m) => self.send_err(&msg.sender_id, errors!( un_implemented a => m ))
        };
        
        None
    }
}
