import * as vscode from 'vscode'

export function toVsRange(doc: vscode.TextDocument, range: [number, number]): vscode.Range {
    return new vscode.Range(
        doc.positionAt(range[0]),
        doc.positionAt(range[1]),
    )
}

export function fromVsRange(doc: vscode.TextDocument, range: vscode.Range): [number, number] {
    return [doc.offsetAt(range.start), doc.offsetAt(range.end)]
}
