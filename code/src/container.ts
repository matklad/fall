import * as vscode from 'vscode'
import { FallTextDocumentContentProvider } from './FallTextDocumentContentProvider'
import { FallDocumentSymbolProvider } from './FallDocumentSymbolProvider'

export let container = {
    uris: {
        syntaxTree: vscode.Uri.parse('fall://syntaxtree'),
        status: vscode.Uri.parse('fall://status')
    },
    textDocumentContentProvider: new FallTextDocumentContentProvider(),
    documetSymbolProvider: new FallDocumentSymbolProvider(),
}