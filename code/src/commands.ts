import * as vscode from 'vscode'

import { State } from './state'
import { container } from './container'
import { toVsRange } from './range'
import { log } from 'util'


export default {
    async status() {
        return openDoc(container.uris.status)
    },

    async showSyntaxTree() {
        return openDoc(container.uris.syntaxTree)
    },

    semanticSelection(state: State) {
        let doc = state.editor.document
        let start = doc.offsetAt(state.editor.selection.start)
        let end = doc.offsetAt(state.editor.selection.end)
        let range = state.file.extendSelection([start, end])
        if (range == null) return
        let [newStart, newEnd] = range
        let newSelection = new vscode.Selection(doc.positionAt(newStart), doc.positionAt(newEnd))
        state.editor.selection = newSelection
    },

    applyContextAction(state, offset, id) {
        // if (offset == -1) {
        //     currentTest = id
        //     textDocumentProvider.eventEmitter.fire(uris.parsedTest)
        //     let document = await vscode.workspace.openTextDocument(uris.parsedTest)
        //     vscode.window.showTextDocument(document, vscode.ViewColumn.Two, true)
        //     return
        // }
        let edits = state.file.applyContextAction(offset, id)
        return state.editor.edit((builder) => {
            for (let op of edits) {
                builder.replace(toVsRange(state.editor.document, op.delete), op.insert)
            }
        })
    }
}

async function openDoc(uri: vscode.Uri) {
    container.textDocumentContentProvider.eventEmitter.fire(uri)
    let document = await vscode.workspace.openTextDocument(uri)
    vscode.window.showTextDocument(document, vscode.ViewColumn.Two, true)
}