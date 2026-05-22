import { EphemeralStore, LoroDoc } from "loro-crdt";
import { Action, type Combination, Message, MessageType } from "./types";
import { ephemeral_protocol, protocol } from "./protocol";

export enum State {
  Disconnected = 'disconnected',
  Retrying = 'retrying',
  Connected = 'connected',
  Connecting = 'connecting',
}

export class Connection {
  private _state: State = State.Disconnected;
  private doc: LoroDoc;
  private ephimeral: EphemeralStore | null = null;
  private ws: WebSocket | null = null;
  
  private retries: number = 0;
  private retryInterval: number = 100;
  private maxRetries: number = Infinity;
  private url: string | URL;
  private protocols: string | string[] | undefined;
  
  private unsubscribe: (() => void) | null = null;
  private eunsubscribe: (() => void) | null = null;

  constructor(
    url: string | URL,
    doc: LoroDoc,
    ephimeral?: EphemeralStore,
    protocols?: string | string[] | undefined,
  ) {
    this.doc = doc;
    this.url = url;
    if (ephimeral) this.ephimeral = ephimeral;
    this.protocols = protocols;
    
    this.changeStatus(State.Disconnected);
  }

  private connect() {
    this.ws = new WebSocket(this.url, this.protocols);
    this.changeStatus(State.Connecting);

    this.ws.onopen = this.onopen.bind(this);
    this.ws.onmessage = this.onmessage.bind(this);
    this.ws.onerror = this.onerror.bind(this);
    this.ws.onclose = this.onclose.bind(this);
  }

  private onopen(ev: Event) {
    this.changeStatus(State.Connected);
    this.retries = 0;
    
    this.unsubscribe = this.doc.subscribeLocalUpdates((c) => this.updates(c))
    
    if (this.ephimeral) {
      this.eunsubscribe = this.ephimeral?.subscribeLocalUpdates(c => this.ephUpdates(c))
    }
    
    const version = this.doc.oplogVersion().encode();
    const message = new Message(MessageType.VersionVector, Action.Answer, version);
    this.send(message);
    if (this.ephimeral) {
      this.send(new Message(MessageType.Ephimeral, Action.Passthrough, new Uint8Array()))
    }
  }
  
  private updates(content: Uint8Array) {
    this.send(new Message(MessageType.Export, Action.Replicate, content))
  }
  
  private ephUpdates(content: Uint8Array) {
    this.send(new Message(MessageType.Ephimeral, Action.Passthrough, content))
  }

  private async onmessage(ev: MessageEvent) {
    const data = ev.data;
    if (data instanceof Uint8Array) this.proccess(data as Uint8Array);
    else if (data instanceof Blob) {
      const arrayBuffer = await data.arrayBuffer();
      this.proccess(new Uint8Array(arrayBuffer));
    }
    else if (typeof data === 'string') this.error(data);
    else console.warn("[!] data", ev.data);
  }

  private onclose(ev: CloseEvent) {
    if (this._state == State.Disconnected) return;
    if (this.unsubscribe) {
      this.unsubscribe();
      this.unsubscribe = null;
    }
    
    if (this.eunsubscribe) {
      this.eunsubscribe();
      this.eunsubscribe = null;
    }
    
    this.ws = null;
    
    switch (ev.code) {
      case 1001:
      case 1002:
      case 1003:
      case 1006:
        this.changeStatus(State.Retrying);
        this.retry();
        break;
      case 3000:
        this.changeStatus(State.Disconnected);
        break;
      default:
        this.changeStatus(State.Disconnected);
        console.warn(`[Connection closed] Code: ${ev.code}, Reason: ${ev.reason}`);
    } 
  }
  private onerror(ev: Event) {
    console.error("error: ", ev);
    if (this._state == State.Connecting)
      this.changeStatus(State.Retrying);
    else this.changeStatus(State.Disconnected);
    this.ws = null;
  }

  private retry() {
    if (this._state != State.Retrying) return;
    if (this.retries >= this.maxRetries) {
      console.error("Max retries reached. Giving up.");
      return;
    }
    this.retries++;
    console.warn(`Retrying connection in ${this.retryInterval}ms... (Attempt ${this.retries}/${this.maxRetries})`);

    setTimeout(() => {
      if (this._state === State.Retrying) this.connect();
    }, this.retryInterval);
  }
  
  private send(message: Message) {
    const stream = message.encode();
    
    if (this.ws && this.ws.readyState === WebSocket.OPEN) {
      console.debug("[send] ", stream);
      this.ws.send(stream);
    }
    else {
      console.debug("[dico] ", stream);
    }
  }
  
  private proccess(bytes: Uint8Array) {
    console.debug("[get ] ", bytes);
    let message = Message.from(bytes);
    let combination: Combination = message.to_combination();
    
    if (protocol[combination]) {
      let response = protocol[combination](this.doc, message.content)
      if (response) this.send(response);
    }
    else if (ephemeral_protocol[combination] && this.ephimeral) {
      let response = ephemeral_protocol[combination](this.ephimeral, message.content)
      if (response) this.send(response);
    }
    else this.error(`Unsupported combination: '${combination}'`)
  }
  
  private error(data: String) {
    console.error("error: ", data)
  }
  
  tryconnect() {
    this.changeStatus(State.Retrying);
    this.connect();
  }
  
  close() {
    this.changeStatus(State.Disconnected);
    if (this.unsubscribe) {
      this.unsubscribe();
      this.unsubscribe = null;
    }
    
    if (this.eunsubscribe) {
      this.eunsubscribe();
      this.eunsubscribe = null;
    }
    
    if (this.ws && (this.ws.readyState === WebSocket.OPEN || this.ws.readyState === WebSocket.CONNECTING)) {
      this.ws.close(3000, "Self killed");
    }
    
    else if (this.ws) console.warn(`Attempted to close WebSocket, but it was in state: ${this.ws.readyState}.`);
    
    this.ws = null;
  }
  
  changeStatus(status: State) {
    this._state = status;
    const event = new CustomEvent('guitite:status-changed', { detail: { status } });
    document.dispatchEvent(event);
  }
  
  get state(): State { return this._state }
}
