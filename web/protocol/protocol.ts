import { LoroDoc, VersionVector } from "loro-crdt";
import { Action, Combination, Message, MessageType } from "./types";


export const protocol: { [key in Combination]?: (doc: LoroDoc, content: Uint8Array) => Message | void } = {
  [`${MessageType.VersionVector}-${Action.Answer}`]: (doc, content) => {
    let update = doc.export({ mode: "update", from: VersionVector.decode(content) });
    return new Message(MessageType.Export, Action.None, update);
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