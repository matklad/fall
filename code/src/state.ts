import * as vscode from 'vscode'
import { LangSupport, VsFile } from './backend'
import { log } from 'util';

export class State {
    editor: vscode.TextEditor
    support: LangSupport
    file: VsFile

    static current: State | null = null

    private constructor(editor: vscode.TextEditor, support: LangSupport, file: VsFile) {
        this.editor = editor
        this.support = support
        this.file = file
    }

    static fromEditor(editor: vscode.TextEditor): State | null {
        let filename = editor.document.fileName
        let idx = filename.lastIndexOf('.')
        if (idx == -1) return null
        let extension = filename.substr(idx + 1)
        let support = LangSupport.forExtension(extension)
        if (support == null) return null
        let file = support.parse(editor.document.getText())
        return new State(editor, support, file)
    }

    getText(): string {
        return this.editor.document.getText()
    }
}
