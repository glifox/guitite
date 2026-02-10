use std::time::{Duration, Instant};
use actix::dev::ContextFutureSpawner;
use actix_web_actors::ws::{WebsocketContext, ProtocolError, Message as WsMessage};
use actix::{Actor, ActorContext, ActorFutureExt, Addr, AsyncContext, Handler, Running, StreamHandler, WrapFuture};
use actix_ws::CloseReason;
use rand::Rng as _;

use super::super::parser::Parse;
use crate::structs::actors::server::Server;
use crate::structs::messages::{Connect, Disconnect, Error, Message, Mresult};


struct Durations { heardbeat: Duration, timeout: Duration }
const DURATIONS: Durations = Durations { 
    heardbeat: Duration::from_secs(5),
    timeout: Duration::from_secs(10)
};

pub struct Client {
    pub(super) id: u64,
    pub(super) last_hb: Instant,
    pub(super) file_name: String,
    pub(super) server: Addr<Server>,
}

impl Client {
    pub fn new<T: Into<String>>(file: T, server: Addr<Server>) -> Self {
        Self { 
            id: rand::rng().random(), 
            last_hb: Instant::now(),
            server, 
            file_name: file.into() 
        }
    }
    
    fn pinging(&self, ctx: &mut WebsocketContext<Self>) {
        ctx.run_interval(DURATIONS.heardbeat, | client, ctx| {
            if Instant::now().duration_since(client.last_hb) > DURATIONS.timeout {
                client.server.do_send(
                    Disconnect { 
                        id: client.id,
                        file: client.file_name.clone() 
                    }
                );
                ctx.stop(); // stop actor
                return;
            }

            ctx.ping(b"");
        });
    }
}

impl Actor for Client { 
    type Context = WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.pinging(ctx);
        
        let connect = Connect { 
            id: self.id,
            file: self.file_name.clone(), 
            addr_msg: ctx.address().recipient(),
            addr_err: ctx.address().recipient() 
        };
        
        self.server.send(connect).into_actor(self)
            .then(|res, _, ctx| { 
                if let Err(_) = res { ctx.stop() }
                
                actix::fut::ready(()) // resolve the future
            })
            .wait(ctx); // wait until the future resolves
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.server.do_send(
            Disconnect { 
                id: self.id,
                file: self.file_name.clone()
            }
        );
        Running::Stop
    }
}

/// WebSocket message handler
impl StreamHandler<Result<WsMessage, ProtocolError>> for Client {
    fn handle(&mut self, msg: Result<WsMessage, ProtocolError>, ctx: &mut Self::Context) {
        
        let msg = match msg {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };

        match msg {
            WsMessage::Pong(_) => self.last_hb = Instant::now(),
            WsMessage::Ping(msg) => {
                self.last_hb = Instant::now();
                ctx.pong(&msg);
            }
            WsMessage::Text(_) => { todo!() }
            WsMessage::Binary(binary) => {
                let (mtype, action) = match binary.to_vec().parse() {
                    Ok(u) => u,
                    Err(e) => { 
                        ctx.text(e); 
                        return; 
                    }
                };
                
                let con = Message {
                    id: self.id,
                    file: self.file_name.clone(),
                    mtype,
                    action
                };
                
                self.server.do_send(con);
            },
            WsMessage::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            WsMessage::Continuation(_) => ctx.stop(),
            WsMessage::Nop => (),
        }
    }
}

impl Handler<Message> for Client {
    type Result = Mresult;

    fn handle(&mut self, msg: Message, ctx: &mut Self::Context) -> Self::Result {
        let action = msg.action;
        
        match msg.mtype.to_send(action) {
            Some(m) => ctx.binary(m),
            None => (),
        };
        
        None
    }
}

impl Handler<Error> for Client {
    type Result = ();

    fn handle(&mut self, msg: Error, ctx: &mut Self::Context) -> Self::Result {
        match msg.fatal {
            true => {
                let reason = CloseReason {
                    code: actix_ws::CloseCode::Error,
                    description: Some(msg.to_string())
                };
                
                ctx.close(Some(reason));
                ctx.stop();
            },
            false => ctx.text(msg.to_string()),
        }
    }
}
