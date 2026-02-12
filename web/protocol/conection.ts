import { LoroDoc } from "loro-crdt";
import { Action, Combination, Message, MessageType } from "./types";

export class Conection {
  private doc: LoroDoc;
  private ws: WebSocket | null = null;
  
  private retries: number = 0;
  private retryInterval: number = 3000;
  private maxRetries: number = Infinity;
  private url: string | URL;
  private protocols: string | string[] | undefined;
  private tryopen: boolean = true;
  
  private unsubscribe: (() => void) | null = null;

  constructor(
    doc: LoroDoc,
    url: string | URL,
    protocols?: string | string[] | undefined,
  ) {
    this.doc = doc;
    this.url = url;
    this.protocols = protocols;
    this.tryopen = true;
    
    this.connect();
  }

  connect() {
    this.ws = new WebSocket(this.url, this.protocols);

    this.ws.onopen = this.onopen.bind(this);
    this.ws.onmessage = this.onmessage.bind(this);
    this.ws.onerror = this.onerror.bind(this);
    this.ws.onclose = this.onclose.bind(this);
    
    this.unsubscribe = this.doc.subscribeLocalUpdates(
      (m) => this.send(new Message(MessageType.Export, Action.Replicate, m))
    )
  }

  private onopen(ev: Event) {
    this.retries = 0;
    
    const version = this.doc.oplogVersion().encode();
    const message = new Message(MessageType.VersionVector, Action.Answer, version);
    // const message = new Message(MessageType.VersionVector, Action.Passthrough, version);
    
    this.send(message);
  }

  private async onmessage(ev: MessageEvent) {
    const data = ev.data;
    if (data instanceof Uint8Array) this.proccess(data as Uint8Array);
    else if (data instanceof Blob) {
      const arrayBuffer = await data.arrayBuffer();
      this.proccess(new Uint8Array(arrayBuffer));
    }
    else if (typeof data === 'string') this.error(data);
    else console.warn("[>]", ev.data);
  }

  private onclose(ev: CloseEvent) {
    if (this.unsubscribe) {
      this.unsubscribe();
      this.unsubscribe = null;
    }
    
    this.ws = null;
    
    console.warn(`[Connection closed] Code: ${ev.code}, Reason: ${ev.reason}`);
    if (ev.code === 4000) return;
    if (ev.reason === "Failed to connect") return;
    this.retry(); 
  }
  private onerror(ev: Event) {
    console.warn("error: ", ev);
    this.ws = null; 
    this.retry()
  }

  private retry() {
    if (!this.tryopen) return;
    if (this.retries >= this.maxRetries) {
      console.error("Max retries reached. Giving up.");
      return;
    }
    this.retries++;
    console.warn(`Retrying connection in ${this.retryInterval}ms... (Attempt ${this.retries}/${this.maxRetries})`);

    setTimeout( () => { if (this.tryopen) this.connect(); }, this.retryInterval);
  }
  
  private send(message: Message) {
    const stream = message.encode();
    console.debug("[send] ", stream);
    if (this.ws && this.ws.readyState === WebSocket.OPEN) this.ws.send(stream);
  }
  
  private proccess(bytes: Uint8Array) {
    console.debug("[get ] ", bytes);
    let message = Message.from(bytes);
    let combination: Combination = message.to_combination();
    
    if (actions[combination]) {
      let response = actions[combination](this.doc, message.content)
      if (response) this.send(message);
    }
    else this.error(`Unsupported combination: '${combination}'`)
  }
  
  private error(data: String) {
    console.error("error: ", data)
  }
  
  close() {
    this.tryopen = false;
    if (this.unsubscribe) {
      this.unsubscribe();
      this.unsubscribe = null;
    }
    
    if (this.ws && (this.ws.readyState === WebSocket.OPEN || this.ws.readyState === WebSocket.CONNECTING)) {
      this.ws.close(4000, "Self killed");
    }
    
    else if (this.ws) console.warn(`Attempted to close WebSocket, but it was in state: ${this.ws.readyState}.`);
    
    this.ws = null;
  }
}

const actions: { [key in Combination]?: (doc: LoroDoc, content: Uint8Array) => Message | undefined } = {
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