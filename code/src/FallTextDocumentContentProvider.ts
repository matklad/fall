import * as vscode from 'vscode'
import { VsFile } from './backend'
import { log } from 'util';

export class FallTextDocumentContentProvider implements vscode.TextDocumentContentProvider {
    file: VsFile
    public eventEmitter = new vscode.EventEmitter<vscode.Uri>()
    public syntaxTree: string = "Not available"

    public provideTextDocumentContent(uri: vscode.Uri): vscode.ProviderResult<string> {
        if (uri.authority == 'syntaxtree') {
            return this.file == null ? "Not availanle" : this.file.syntaxTree()
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
