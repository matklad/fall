import * as vscode from 'vscode'
import { FallTextDocumentContentProvider } from './FallTextDocumentContentProvider'
import { FallDocumentSymbolProvider } from './FallDocumentSymbolProvider'
import { FallCodeActionProvider } from './FallCodeActionProvider'
import { FallDocumentFormattingEditProvider } from './FallDocumentFormattingEditProvider'

export let container = {
    uris: {
        syntaxTree: vscode.Uri.parse('fall://syntaxtree'),
        status: vscode.Uri.parse('fall://status')
    },
    textDocumentContentProvider: new FallTextDocumentContentProvider(),
    documetSymbolProvider: new FallDocumentSymbolProvider(),
    codeActionProvider: new FallCodeActionProvider(),
    documentFormattingEditProvider: new FallDocumentFormattingEditProvider()
}