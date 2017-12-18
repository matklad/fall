import * as vscode from 'vscode'
import { State } from './state'
import { toVsRange } from './range'
import { log } from 'util';

export class FallDocumentFormattingEditProvider implements vscode.DocumentFormattingEditProvider {
    provideDocumentFormattingEdits(
        document: vscode.TextDocument,
        options: vscode.FormattingOptions,
        token: vscode.CancellationToken
    ): vscode.TextEdit[] {
        log("Reformating")
        let state = State.current
        if (state == null) return
        return state.file.reformat().map((op) => {
            return vscode.TextEdit.replace(toVsRange(state.editor.document, op.delete), op.insert)
        })
    }
}

