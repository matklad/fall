import * as vscode from 'vscode'
import { VsFile } from './backend'
import { profileResultsAsString } from './profile'
import { State } from './state'
import { log } from 'util'


export class FallTextDocumentContentProvider implements vscode.TextDocumentContentProvider {
    public eventEmitter = new vscode.EventEmitter<vscode.Uri>()
    public syntaxTree: string = "Not available"

    public provideTextDocumentContent(uri: vscode.Uri): vscode.ProviderResult<string> {
        let state = State.current
        if (state == null) return "Not available"

        if (uri.authority == 'syntaxtree') {
            return state.file.syntaxTree()
        }

        if (uri.authority == 'status') {
            return state.file.metrics() + `\n${profileResultsAsString()}`
        }

        log(`Bad uri: ${uri}`)
    }

    get onDidChange(): vscode.Event<vscode.Uri> {
        return this.eventEmitter.event
    }
}
