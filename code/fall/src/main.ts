'use strict'

import * as vscode from 'vscode'
import { createPlugin } from './common'
import { log } from 'util'

const backend = require("../../native")

const fallDiagnostics = vscode.languages.createDiagnosticCollection("fall")

export function activate(context: vscode.ExtensionContext) {
    let plugin = createPlugin(backend, "fall", context.subscriptions, true, fallDiagnostics)

    let commands = [
        ["fall-fall.semanticSelection", "extendSelection"],
        ["fall-fall.showSyntaxTree", "showSyntaxTree"],
        ["fall-fall.metrics", "metrics"],
    ]
    for (let [key, action] of commands) {
        let cmd = vscode.commands.registerCommand(key, () => plugin[action]())
        context.subscriptions.push(cmd)
    }

    let providers = [
        vscode.workspace.registerTextDocumentContentProvider('fall-fall', plugin.textDocumentContentProvider),
        vscode.languages.registerCodeActionsProvider("fall", plugin.codeActionProvider),
        vscode.languages.registerDocumentSymbolProvider("fall", plugin.documentSymbolsProvider),
        vscode.languages.registerDocumentFormattingEditProvider("fall", plugin.documentFormattingEditProvider),
    ]
    context.subscriptions.push(...providers)
}


export function deactivate() { }
