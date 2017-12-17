'use strict';

import * as vscode from 'vscode'
import commands from './commands'
import { LangSupport } from './backend'
import { State } from './state'
import { container } from './container'
import { setHighlights } from './highlight'
import { log } from 'util'


var current: State | null = null


export function activate(context: vscode.ExtensionContext) {
    log("Activating extension")
    for (let name in commands) {
        let callback = commands[name]
        let cmd = vscode.commands.registerCommand("fall." + name, () => {
            let state = current
            if (current == null) return
            log(`Command ${name}`)
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

    vscode.workspace.onDidChangeTextDocument(event => {
        if (["fall", "rust-fall"].indexOf(event.document.languageId) != -1) {
            if (event.contentChanges.length == 1) {
                let edits = event.contentChanges.map((change) => {
                    let start = event.document.offsetAt(change.range.start)
                    return {
                        "delete": [start, start + change.rangeLength],
                        "insert": change.text
                    }
                })
                log("Incremental reparse")
                updateState(vscode.window.activeTextEditor, edits)
            } else {
                log("Full reparse")
                resetState(vscode.window.activeTextEditor)
            }
        }
    }, null, context.subscriptions)
    log("Set up listeners")

    vscode.window.onDidChangeActiveTextEditor(resetState)
    resetState(vscode.window.activeTextEditor)
    log("Extension activated")
}

function updateState(editor: vscode.TextEditor, edits) {
    if (current.editor != editor || current.file == null) {
        resetState(editor)
        return
    }
    current.file = current.file.edit(edits)
    afterStateUpdate(current)
}

function resetState(editor: vscode.TextEditor) {
    if (editor == null) {
        current = null
        return
    }
    current = State.fromEditor(editor)
    afterStateUpdate(current)
}

function afterStateUpdate(state: State) {
    log("Start afterStateUpdate")
    let tdcp = container.textDocumentContentProvider
    tdcp.updateFile(current.file)
    tdcp.eventEmitter.fire(container.uris.syntaxTree)
    tdcp.eventEmitter.fire(container.uris.status)
    setHighlights(state.editor, state.file.highlight())
    log("Finish afterStateUpdate")
}

export function deactivate() { }