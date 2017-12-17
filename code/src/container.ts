import * as vscode from 'vscode'
import {FallTextDocumentContentProvider} from './FallTextDocumentContentProvider'

export let container = {
    uris: {
        syntaxTree: vscode.Uri.parse('fall://syntaxtree'),
        status: vscode.Uri.parse('fall://status')
    },
    textDocumentContentProvider: new FallTextDocumentContentProvider()
}