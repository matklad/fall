import * as vscode from 'vscode'

import {State} from './state'
import {container} from './container'

export default {
    async status() {
        return openDoc(container.uris.status)
    },

    async showSyntaxTree() {
        return openDoc(container.uris.syntaxTree)
    }
}

async function openDoc(uri: vscode.Uri) {
    container.textDocumentContentProvider.eventEmitter.fire(uri)
    let document = await vscode.workspace.openTextDocument(uri)
    vscode.window.showTextDocument(document, vscode.ViewColumn.Two, true)
}