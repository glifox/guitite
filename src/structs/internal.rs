use std::borrow::Borrow;
use std::fmt::Display;
use std::hash::{Hash, Hasher};
use actix::prelude::Recipient;

use crate::structs::messages::{Disconnect, Message};

#[derive(Clone)]
pub enum Action {
    None,
    Replicate,
    Answer,
    Passthrough
}

impl Action {
    fn byte(&self) -> u8 {
        match self {
            Action::None => 0,
            Action::Replicate => 1,
            Action::Answer => 2,
            Action::Passthrough => 3,
        }
    }
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Action::")?;
        match self {
            Action::None => write!(f, "None"),
            Action::Replicate => write!(f, "Replicate"),
            Action::Answer => write!(f, "Answer"),
            Action::Passthrough => write!(f, "Passthrough"),
        }?;
        
        write!(f, " ({})", self.byte())
    }
}

#[derive(Clone)]
pub enum MessageType {
    None,
    Export(Vec<u8>),
    VersionVector(Vec<u8>),
    Frontiers(Vec<u8>),
    Ephimeral(Vec<u8>)
}

impl MessageType {
    pub(crate) fn to_send(&self, action: Action) -> Option<Vec<u8>> {
        match self {
            MessageType::None => None,
            MessageType::Export(items) => {
                Some(Self::to_vec(1, action.byte(), items))
            },
            MessageType::VersionVector(items) => {
                Some(Self::to_vec(2, action.byte(), items))
            },
            MessageType::Frontiers(items) => {
                Some(Self::to_vec(3, action.byte(), items))
            },
            MessageType::Ephimeral(items) => {
                Some(Self::to_vec(4, action.byte(), items))
            },
        }
    }
    
    fn to_vec(byte1: u8, byte2: u8, vec: &Vec<u8>) -> Vec<u8> {
        let mut vec = vec.clone();
        
        vec.insert(0, byte1);
        vec.insert(1, byte2);
        
        vec
    }
}

impl Display for MessageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MessageType::")?;
        match self {
            MessageType::None => write!(f, "None (0)"),
            MessageType::Export(_) => write!(f, "Export (1)"),
            MessageType::VersionVector(_) => write!(f, "VersionVector (2)"),
            MessageType::Frontiers(_) => write!(f, "Frontiers (3)"),
            MessageType::Ephimeral(_) => write!(f, "Ephimeral (4)"),
        }
    }
}

#[derive(Eq)]
pub struct File {
    pub name: String,
    pub message: Recipient<Message>,
    pub disconnect: Recipient<Disconnect>,
}

impl Hash for File {
    fn hash<H: Hasher>(&self, state: &mut H) { self.name.hash(state); }
}

impl PartialEq for File {
    fn eq(&self, other: &Self) -> bool { self.name == other.name }
}

impl Borrow<str> for File {
    fn borrow(&self) -> &str { &self.name }
}

impl Borrow<String> for File {
    fn borrow(&self) -> &String { &self.name }
}
