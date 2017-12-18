import * as vscode from 'vscode'
import { State } from './state'
import { fromVsRange } from './range'
import { log } from 'util';

export class FallCodeActionProvider implements vscode.CodeActionProvider {
    provideCodeActions(
        document: vscode.TextDocument,
        _range: vscode.Range,
        context: vscode.CodeActionContext,
        token: vscode.CancellationToken
    ): vscode.Command[] {
        let state = State.current
        if (state == null) return
        let range = fromVsRange(document, _range)

        // let test = currentFile.testAtOffset(range[0]);
        // if (test != null) {
        //     return [{
        //         title: "Parse test",
        //         command: 'extension.applyContextAction',
        //         arguments: [-1, test]
        //     }]
        // }

        let actions = state.file.contextActions(range)
        log(`Available actions ${actions.join(",")}`)
        return actions.map((id) => {
            return {
                title: id,
                command: 'fall.applyContextAction',
                arguments: [range, id]
            }
        })
    }
}
