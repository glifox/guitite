import { LoroDoc, VersionVector, EphemeralStore } from "loro-crdt";
import { Action, Combination, Message, MessageType } from "./types";


export const protocol: { [key in Combination]?: (doc: LoroDoc, content: Uint8Array) => Message | void } = {
  [`${MessageType.VersionVector}-${Action.Answer}`]: (doc, content) => {
    let update: Uint8Array;
    // if (content.length === 1 && content[0] === 0) update = doc.export({ mode: "update" });
    // else 
    update = doc.export({ mode: "update", from: VersionVector.decode(content) });
    
    return new Message(MessageType.Export, Action.Replicate, update);
  },
  
  [`${MessageType.None}-${Action.Answer}`]: (doc, _) => {
    const update = doc.export({ mode: "update" });
    return new Message(MessageType.Export, Action.None, update);
  },
  
  [`${MessageType.Export}-${Action.None}`]: (doc, content) => {
    const status = doc.import(content);
    if (status.pending) {
      return new Message(MessageType.VersionVector, Action.Answer, doc.version().encode())
    }
  },
  
  // [`${MessageType.Export}-${Action.Replicate}`]: (doc, content) => {  },
}

export const ephimeral_protocol: { [key in Combination]?: (store: EphemeralStore, content: Uint8Array) => Message | void } = {
  [`${MessageType.Ephimeral}-${Action.None}`]: (store, content) => { 
    if (content.length == 0) { return new Message(MessageType.Ephimeral, Action.Replicate, store.encodeAll()) }
    store.apply(content);
  },
  
  // [`${MessageType.Export}-${Action.Replicate}`]: (store, content) => {  },
}