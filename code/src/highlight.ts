import * as vscode from 'vscode'
import { toVsRange } from './range'
import { log } from 'util'

const decorations = (() => {
    const decor = (obj) => vscode.window.createTextEditorDecorationType({ color: obj })
    return {
        background: decor("#3F3F3F"),
        error: vscode.window.createTextEditorDecorationType({
            borderColor: "red",
            borderStyle: "none none dashed none",
        }),
        comment: decor("#7F9F7F"),
        string: decor("#CC9393"),
        keyword: decor("#F0DFAF"),
        function: decor("#93E0E3"),
        parameter: decor("#94BFF3"),
        builtin: decor("#DD6718"),
        text: decor("#DCDCCC"),
        attribute: decor("#BFEBBF"),
        literal: decor("#DFAF8F"),
    }
})()

export function setHighlights(
    editor: vscode.TextEditor,
    highlihgs: Array<[[number, number], string]>
) {
    let byTag = {}
    for (let tag in decorations) {
        byTag[tag] = []
    }

    for (let [_range, tag] of highlihgs) {
        if (!byTag[tag]) {
            log(`unknown tag ${tag}`)
            continue
        }
        let range = toVsRange(editor.document, _range)
        byTag[tag].push(range)
    }

    for (let tag in byTag) {
        let dec = decorations[tag]
        let ranges = byTag[tag]
        editor.setDecorations(dec, ranges)
    }
}
