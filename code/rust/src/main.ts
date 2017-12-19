'use strict'

import * as vscode from 'vscode'
import { createPlugin } from './common'
import { log } from 'util'

const backend = require("../../native")


export function activate(context: vscode.ExtensionContext) {
    let plugin = createPlugin(backend, "rs", context.subscriptions)

    let commands = [
        ["fall-rust.semanticSelection", "extendSelection"],
        ["fall-rust.showSyntaxTree", "showSyntaxTree"],
        ["fall-rust.metrics", "metrics"],
    ]
    for (let [key, action] of commands) {
        let cmd = vscode.commands.registerCommand(key, () => plugin[action]())
        context.subscriptions.push(cmd)
    }

    let providers = [
        vscode.workspace.registerTextDocumentContentProvider('fall-rs', plugin.textDocumentContentProvider),
        vscode.languages.registerDocumentSymbolProvider('rust', plugin.documentSymbolsProvider),

    ]
    context.subscriptions.push(...providers)
}


export function deactivate() { }
