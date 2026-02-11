use actix::prelude::{Recipient, Message as msg};

use crate::structs::internal::{Action, MessageType};

#[derive(Clone, Debug)]
pub enum Target {
    All(String),
    Client(u64)
}

#[derive(msg, Clone, Debug)]
#[rtype(result = "()")]
pub struct Message {
    /// Sender id
    pub id: u64,
    pub file: String,
    pub mtype: MessageType,
    pub action: Action,
}

#[derive(msg, Clone, Debug)]
#[rtype(result = "()")]
pub struct Response {
    pub target: Target,
    pub response: Result<Message, Error>
}

#[derive(msg, Clone, Debug)]
#[rtype(result = "()")]
pub struct Connect {
    pub id: u64,
    pub file: String,
    pub addr_msg: Recipient<Message>,
    pub addr_err: Recipient<Error>,
}

#[derive(msg, Clone, Debug)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: u64,
    pub file: String,
}

#[derive(msg, Clone, Debug)]
#[rtype(result = "()")]
pub struct Error {
    pub status: u16,
    pub message: String,
    pub fatal: bool,
}

impl ToString for Error {
    fn to_string(&self) -> String {
        format!("{}: {}", self.status, self.message)
    }
}
