pub use guitite_derive::Protocol;

use std::borrow::Cow;
use loro::{ExportMode, LoroDoc, VersionVector};

use crate::structs::conversor::Conversor;
use crate::structs::internal::{Action, MessageType};
use crate::structs::messages::{Error, Message};
use crate::{errors, message};


pub trait Protocol {
    fn version_vector(&self, doc: &LoroDoc, message: Message) -> Message {
        let version = doc.oplog_vv().encode();
        message!(copy message, MessageType::VersionVector(version), Action::Answer)
    }
    
    fn process(&self, doc: &LoroDoc, message: Message) -> Result<Option<Message>, Error> {        
        match (&message.mtype, &message.action) {
            (MessageType::VersionVector(vv), Action::Answer) => {
                self.on_version_vector(vv);
                let version: VersionVector = VersionVector::decode(vv).map_err(|e| e.to_error())?;
                let version = Cow::Borrowed(&version);
                let update = doc.export(ExportMode::Updates { from: version }).map_err(|e| e.to_error())?;
                Ok(Some(message!(copy message, MessageType::Export(update), Action::None)))
            }
            (MessageType::Export(bytes), Action::None) => {
                self.on_export(bytes);
                let status = doc.import(bytes).map_err(|e| e.to_error())?;
                let msg = status.pending.map(|_| { self.version_vector(doc, message) });
                Ok(msg)
            }
            (MessageType::Ephimeral(_), Action::None) => Ok(None),
            (a, m) => Err(errors!( un_implemented a => m ))
        }
    }
    
    #[allow(unused_variables)]
    fn on_version_vector(&self, vv: &Vec<u8>) {}
    
    #[allow(unused_variables)]
    fn on_export(&self, bytes: &Vec<u8>) {}
}
