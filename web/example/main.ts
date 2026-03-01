import { EditorState } from "@codemirror/state";
import { dropCursor, EditorView } from "@codemirror/view";
import { LoroExtensions } from "loro-codemirror";
import { EphemeralStore, LoroDoc, UndoManager } from "loro-crdt";
import { Connection, State } from "@glifox/guitite";

document.addEventListener("guitite:status-changed", e => {
  document.querySelector("#st")!.textContent = (e as CustomEvent).detail.status;
});

const doc = new LoroDoc();
const ephemeral = new EphemeralStore();
const con = new Connection("ws://localhost:8080/ws/some", doc, ephemeral);

document.querySelector("#cn")!.addEventListener('click', () => {console.info("cn:");con.tryconnect()});
document.querySelector("#dc")!.addEventListener('click', () => {console.info("ds:");con.close()});

const undoManager = new UndoManager(doc, {});

new EditorView({
    state: EditorState.create({
        extensions: [
            // ... other extensions
            LoroExtensions(
                doc,
                // optional LoroEphemeralPlugin
                {
                    ephemeral,
                    user: { name: "Bob", colorClassName: "user1" },
                },
                // optional LoroUndoPlugin
                undoManager,
            ),
        ],
    }),
    parent: document.querySelector("#editor")!,
});
