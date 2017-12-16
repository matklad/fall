import * as vscode from 'vscode'

import {backend} from './backend'
import {State} from './state'
import {container} from './container'

export default {
    status() {
        let status = backend.status()
        console.log(status)
    },
    
    async showSyntaxTree(state: State) {
        let tree = state.file.syntaxTree()
        container.textDocumentContentProvider.syntaxTree = tree
        container.textDocumentContentProvider.eventEmitter.fire(container.uris.syntaxTree)
        let document = await vscode.workspace.openTextDocument(container.uris.syntaxTree)
        vscode.window.showTextDocument(document, vscode.ViewColumn.Two, true)
    }
}