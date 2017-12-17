import * as vscode from 'vscode'

export function toVsRange(doc: vscode.TextDocument, range: [number, number]) {
    return new vscode.Range(
        doc.positionAt(range[0]),
        doc.positionAt(range[1]),
    )
}

