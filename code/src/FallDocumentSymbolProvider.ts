import * as vscode from 'vscode'
import { State } from './state'
import { toVsRange } from './range'

export class FallDocumentSymbolProvider implements vscode.DocumentSymbolProvider {
    provideDocumentSymbols(document: vscode.TextDocument, token: vscode.CancellationToken) {
        let state = State.current
        if (state == null) return
        return state.file.structure().map((node) => {
            return new vscode.SymbolInformation(
                node.name,
                vscode.SymbolKind.Function,
                toVsRange(document, node.range),
                null,
                null
            )
        })
    }
}
