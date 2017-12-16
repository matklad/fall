'use strict';

import * as vscode from 'vscode'
import commands from './commands'
import { backend, LangSupport } from './backend'
import { State } from './state'


var current: State | null = null


export function activate(context: vscode.ExtensionContext) {
    for (let name in commands) {
        let callback = commands[name]
        vscode.commands.registerCommand("fall." + name, () => {
            let state = current
            if (current == null) return
            callback(current)
        })
    }

    vscode.window.onDidChangeActiveTextEditor(changeEditor)
    changeEditor(vscode.window.activeTextEditor)
}

function changeEditor(editor: vscode.TextEditor) {
    if (editor == null) {
        current = null
        return
    }
    current = State.fromEditor(editor)
}

export function deactivate() { }