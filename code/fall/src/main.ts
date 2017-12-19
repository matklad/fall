'use strict'

import * as vscode from 'vscode'
import { createPlugin, EditorFile, toVsRange } from './common'
import { log } from 'util'
import { connect } from 'net';

const backend = require("../../native")

const fallDiagnostics = vscode.languages.createDiagnosticCollection("fall")

export function activate(context: vscode.ExtensionContext) {
    let plugin = createPlugin(backend, "fall", context.subscriptions, true, fallDiagnostics)
    let tdcp = new TextDocumentContentProvider(plugin.getFile)

    let commands = [
        ["fall-fall.semanticSelection", "extendSelection"],
        ["fall-fall.showSyntaxTree", "showSyntaxTree"],
        ["fall-fall.metrics", "metrics"],
    ]
    for (let [key, action] of commands) {
        let cmd = vscode.commands.registerCommand(key, () => plugin[action]())
        context.subscriptions.push(cmd)
    }
    context.subscriptions.push(
        vscode.commands.registerCommand("fall-fall.renderTest", async (test_n, file) => {
            tdcp.test_n = test_n
            let document = await vscode.workspace.openTextDocument(tdcp.uri)
            return vscode.window.showTextDocument(document, vscode.ViewColumn.Two, true)
        })
    )

    vscode.workspace.onDidSaveTextDocument(() => {
        tdcp.eventEmitter.fire(tdcp.uri)
    })

    let providers = [
        vscode.workspace.registerTextDocumentContentProvider('fall-fall', plugin.textDocumentContentProvider),
        vscode.workspace.registerTextDocumentContentProvider('fall-tests', tdcp),
        vscode.languages.registerCodeActionsProvider("fall", plugin.codeActionProvider),
        vscode.languages.registerCodeActionsProvider("fall", new CodeActionProvider(plugin.getFile)),
        vscode.languages.registerDocumentSymbolProvider("fall", plugin.documentSymbolsProvider),
        vscode.languages.registerDocumentFormattingEditProvider("fall", plugin.documentFormattingEditProvider),
        vscode.languages.registerReferenceProvider("fall", new ReferenceProvider(plugin.getFile)),
        vscode.languages.registerDefinitionProvider("fall", new DefinitionProvider(plugin.getFile)),
    ]
    context.subscriptions.push(...providers)
}

class DefinitionProvider implements vscode.DefinitionProvider {
    getFile: (doc: vscode.TextDocument) => EditorFile | null;
    constructor(getFile) {
        this.getFile = getFile
    }

    provideDefinition(
        document: vscode.TextDocument,
        position: vscode.Position,
        token: vscode.CancellationToken
    ): vscode.Location {
        let file = this.getFile(document)
        if (file == null) return
        let range = file.call("resolveReference", document.offsetAt(position))
        if (range == null) return null
        let vsRange =  toVsRange(document, range)
        return new vscode.Location(document.uri, vsRange)
    }
}

class ReferenceProvider implements vscode.ReferenceProvider {
    getFile: (doc: vscode.TextDocument) => EditorFile | null;
    constructor(getFile) {
        this.getFile = getFile
    }
    provideReferences(
        document: vscode.TextDocument,
        position: vscode.Position,
        context: vscode.ReferenceContext,
        token: vscode.CancellationToken
    ): vscode.Location[] {
        let file = this.getFile(document)
        if (file == null) return

        return file.call("findUsages", document.offsetAt(position))
            .map((range) => new vscode.Location(document.uri, toVsRange(document, range)))
    }
}

class CodeActionProvider implements vscode.CodeActionProvider {
    getFile: (doc: vscode.TextDocument) => EditorFile | null;
    constructor(getFile) {
        this.getFile = getFile
    }

    provideCodeActions(
        document: vscode.TextDocument,
        range: vscode.Range,
        context: vscode.CodeActionContext,
        token: vscode.CancellationToken
    ): vscode.Command[] {
        let file = this.getFile(document)
        if (file == null) return

        let test_n = file.call("testAtOffset", document.offsetAt(range.start))
        if (test_n == null) return
        return [
            {
                title: "Render test",
                command: "fall-fall.renderTest",
                arguments: [test_n, file]
            }
        ]
    }
}

class TextDocumentContentProvider implements vscode.TextDocumentContentProvider {
    uri = vscode.Uri.parse("fall-tests://render")
    test_n
    getFile: (doc: vscode.TextDocument) => EditorFile | null;
    constructor(getFile) {
        this.getFile = getFile
    }

    public eventEmitter = new vscode.EventEmitter<vscode.Uri>()

    public provideTextDocumentContent(uri: vscode.Uri): vscode.ProviderResult<string> {
        if (uri.toString() == this.uri.toString()) {
            let doc = vscode.window.activeTextEditor.document
            let file = this.getFile(doc)
            return new Promise((resolve, reject) => {
                file.call("renderTest", this.test_n, (err, text) => {
                    if (err) return reject(err)
                    resolve(text)
                })
            })
        }
        log(`Bad uri: ${uri}`)
    }

    get onDidChange(): vscode.Event<vscode.Uri> {
        return this.eventEmitter.event
    }
}
export function deactivate() { }
