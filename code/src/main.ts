'use strict';

import * as vscode from 'vscode'
import commands from './commands'
import { backend, LangSupport } from './backend'
import { State } from './state'
import { container } from './container'
import { log } from 'util';


var current: State | null = null


export function activate(context: vscode.ExtensionContext) {
    log("Activating extension")
    for (let name in commands) {
        let callback = commands[name]
        let cmd = vscode.commands.registerCommand("fall." + name, () => {
            let state = current
            if (current == null) return
            log(`exec ${name}`)
            return callback(current)
        })
        context.subscriptions.push(cmd)
    }
    log("Registered actions")

    let providers = [
        vscode.workspace.registerTextDocumentContentProvider('fall', container.textDocumentContentProvider),
    ]
    context.subscriptions.push(...providers)
    log("Registered providers")

    vscode.window.onDidChangeActiveTextEditor(changeEditor)
    changeEditor(vscode.window.activeTextEditor)
    log("Extension activated")
}

function changeEditor(editor: vscode.TextEditor) {
    if (editor == null) {
        current = null
        return
    }
    current = State.fromEditor(editor)
}

export function deactivate() { }