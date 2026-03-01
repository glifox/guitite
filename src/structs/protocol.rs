pub use guitite_derive::Protocol;

use std::borrow::Cow;
use loro::{ExportMode, LoroDoc, VersionVector};

use crate::structs::conversor::Conversor;
use crate::structs::internal::{Action, MessageType};
use crate::structs::messages::{Error, Message};
use crate::{errors, message};

/// The default behaviour of the protocol.
/// The only method meant to be rewriten is on_import 
/// to solve thinks like temporal saving.
pub trait Protocol {
    fn version_vector(&self, doc: &LoroDoc, message: Message) -> Message {
        let version = doc.oplog_vv().encode();
        message!(copy message, MessageType::VersionVector(version), Action::Answer)
    }
    
    fn process(&self, doc: &LoroDoc, message: Message) -> Result<Option<Message>, Error> {        
        match (&message.mtype, &message.action) {
            (MessageType::VersionVector(vv), Action::Answer) => {
                let version: VersionVector = VersionVector::decode(vv).map_err(|e| e.to_error())?;
                let version = Cow::Borrowed(&version);
                let update = doc.export(ExportMode::Updates { from: version }).map_err(|e| e.to_error())?;
                Ok(Some(message!(copy message, MessageType::Export(update), Action::None)))
            }
            (MessageType::Export(bytes), Action::None) => {
                self.on_import(bytes);
                let status = doc.import(bytes).map_err(|e| e.to_error())?;
                let msg = status.pending.map(|_| { self.version_vector(doc, message) });
                Ok(msg)
            }
            (MessageType::Ephimeral(_), Action::None) => Ok(None),
            (a, m) => Err(errors!( un_implemented a => m ))
        }
    }
    
    /// This method triggers every time and update is recived. 
    #[allow(unused_variables)]
    fn on_import(&self, bytes: &Vec<u8>) {}
}
