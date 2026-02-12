import { EditorState } from "@codemirror/state";
import { EditorView } from "@codemirror/view";
import { LoroExtensions } from "loro-codemirror";
import { EphemeralStore, LoroDoc, UndoManager } from "loro-crdt";
import { Conection } from "./protocol/conection";

const doc = new LoroDoc();
const con = new Conection(doc, "ws://localhost:8080/ws/some");

document.querySelector("#cn")!.addEventListener('click', () => con.tryconnect());
document.querySelector("#dc")!.addEventListener('click', () => con.close());

const ephemeral = new EphemeralStore();
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