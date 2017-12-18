'use strict';

import * as vscode from 'vscode'
import commands from './commands'
import { LangSupport } from './backend'
import { State } from './state'
import { container } from './container'
import { setHighlights } from './highlight'
import { setDiagnostics } from './diagnostics'
import { log } from 'util'


export function activate(context: vscode.ExtensionContext) {
    log("Activating extension")
    for (let name in commands) {
        let callback = commands[name]
        let cmd = vscode.commands.registerCommand("fall." + name, () => {
            let state = State.current
            if (state == null) return
            log(`Command ${name}`)
            return callback(state)
        })
        context.subscriptions.push(cmd)
    }
    log("Registered actions")

    let providers = [
        vscode.workspace.registerTextDocumentContentProvider('fall', container.textDocumentContentProvider),
        vscode.languages.registerDocumentSymbolProvider('fall', container.documetSymbolProvider),
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
    let current = State.current
    if (current.editor != editor || current.file == null) {
        resetState(editor)
        return
    }
    State.current.file = current.file.edit(edits)
    afterStateUpdate(current)
}

function resetState(editor: vscode.TextEditor) {
    if (editor == null) {
        State.current = null
        return
    }
    State.current = State.fromEditor(editor)
    afterStateUpdate(State.current)
}

function afterStateUpdate(state: State) {
    log("Start afterStateUpdate")
    let tdcp = container.textDocumentContentProvider
    tdcp.eventEmitter.fire(container.uris.syntaxTree)
    tdcp.eventEmitter.fire(container.uris.status)
    setHighlights(state.editor, state.file.highlight())
    setDiagnostics(state.editor.document, state.file.diagnostics())
    log("Finish afterStateUpdate")
}

export function deactivate() { }