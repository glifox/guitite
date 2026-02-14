
export enum Action {
  None = 0,
  Replicate = 1,
  Answer = 2,
  Passthrough = 3,
}

export enum MessageType {
  None = 0,
  Export = 1,
  VersionVector = 2,
  Frontiers = 3,
  Ephimeral = 4,
}

export type Combination = `${MessageType}-${Action}`;

export class Message {
  readonly mtype: MessageType
  readonly action: Action
  readonly content: Uint8Array
  
  constructor(mtype: MessageType, action: Action, content: Uint8Array) {
    this.mtype = mtype; this.action = action; this.content = content;
  }
  
  static from(bytes: Uint8Array): Message {
    return new Message(bytes[0] as MessageType, bytes[1] as Action, bytes.slice(2))
  }
  
  encode(): Uint8Array {
    return Uint8Array.from([this.mtype.valueOf(), this.action.valueOf(), ...this.content])
  }
  
  to_combination(): Combination {
    return `${this.mtype}-${this.action}` as Combination;
  }
}
