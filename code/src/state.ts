import * as vscode from 'vscode'
import { backend, LangSupport } from './backend'

export class State {
    editor: vscode.TextEditor
    support: LangSupport

    constructor(editor: vscode.TextEditor, support: LangSupport) {
        this.editor = editor
        this.support = support
    }

    static fromEditor(editor: vscode.TextEditor): State | null {
        let filename = editor.document.fileName
        let idx = filename.lastIndexOf('.')
        if (idx == -1) return null
        let extension = filename.substr(idx + 1)
        let support = LangSupport.forExtension(extension)
        if (support == null) return null
        return new State(editor, support)
    }
}
