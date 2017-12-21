'use strict'

import * as vscode from 'vscode'
import { createPlugin, toVsRange } from './common'
import { log } from 'util'
import { Uri, SymbolKind } from 'vscode';

const backend = require("../../native")


export function activate(context: vscode.ExtensionContext) {
    let plugin = createPlugin(backend, "rs", context.subscriptions, true)

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
        vscode.languages.registerCodeActionsProvider('rust', plugin.codeActionProvider),
        vscode.languages.registerWorkspaceSymbolProvider(new WorkspaceSymbolProvider(backend)),
    ]
    context.subscriptions.push(...providers)
}

export function deactivate() { }

class WorkspaceSymbolProvider implements vscode.WorkspaceSymbolProvider {
    index;
    backend
    constructor(backend) {
        let folders = vscode.workspace.workspaceFolders
        if (folders == null) folders = []
        let roots = folders.map((f) => f.uri.fsPath)
        this.index = backend.createIndex(roots)
        this.backend = backend
    }

    provideWorkspaceSymbols(query: string, token: vscode.CancellationToken): vscode.SymbolInformation[] {
        let symbols = this.backend.queryIndex(this.index, query)
        let result = []
        for (let [path, symbol] of symbols) {
            function toPos(lineCol) {
                return new vscode.Position(lineCol.line, lineCol.col)
            }
            let uri = Uri.file(path)
            result.push(new vscode.SymbolInformation(
                symbol.name,
                vscode.SymbolKind.Class,
                new vscode.Range(toPos(symbol.lc_range[0]), toPos(symbol.lc_range[1])),
                uri
            ))
        }
        return result
    }
}
