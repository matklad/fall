import * as vscode from 'vscode'
import { VsFile } from './backend'
import { profileResultsAsString } from './profile'
import { log } from 'util'

export class FallTextDocumentContentProvider implements vscode.TextDocumentContentProvider {
    file: VsFile
    public eventEmitter = new vscode.EventEmitter<vscode.Uri>()
    public syntaxTree: string = "Not available"

    public provideTextDocumentContent(uri: vscode.Uri): vscode.ProviderResult<string> {
        let file = this.file
        if (file == null) return "Not available"

        if (uri.authority == 'syntaxtree') {
            return file.syntaxTree()
        }

        if (uri.authority == 'status') {
            return file.metrics() + `\n${profileResultsAsString()}`
        }

        log(`Bad uri: ${uri}`)
    }

    get onDidChange(): vscode.Event<vscode.Uri> {
        return this.eventEmitter.event
    }

    public updateFile(file: VsFile) {
        this.file = file
    }
}
