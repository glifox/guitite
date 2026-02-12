import { LoroDoc } from "loro-crdt";
import { Action, Combination, Message, MessageType } from "./types";


export const protocol: { [key in Combination]?: (doc: LoroDoc, content: Uint8Array) => Message | undefined } = {
  [`${MessageType.None}-${Action.Answer}`]: (doc, _) => {
    const update = doc.export({ mode: "update" });
    return new Message(MessageType.Export, Action.None, update);
  },
  
  [`${MessageType.Export}-${Action.None}`]: (doc, content) => {
    const status = doc.import(content);
    if (status.pending) {
      return new Message(MessageType.VersionVector, Action.Answer, doc.version().encode())
    }
  }
}