'use strict';
import * as vscode from 'vscode';
import { window, DecorationRenderOptions, StatusBarAlignment } from 'vscode';

var backend = (() => {
    var native = require('../../native')
    return {
        create: (text) => native.file_create(text),
        highlight: () => native.file_highlight(),
        stats: () => {
            let stats = native.file_stats()
            if (stats == null) return stats
            return {
                lexing_time: stats.lexing_time,
                parsing_time: stats.parsing_time,
                reparse_range: [stats.reparse_start, stats.reparse_end]
            }
        },
        extendSelection: ([start, end]) => {
            return native.file_extend_selection(start, end);
        },
        tree: (): string => native.file_tree()
    }
})()

export function activate(context: vscode.ExtensionContext) {
    var status = window.createStatusBarItem(StatusBarAlignment.Left)

    var activeEditor = window.activeTextEditor;

    const decor = (obj) => vscode.window.createTextEditorDecorationType({ color: obj })
    const decorations = {
        background: decor("#3F3F3F"),
        meta: decor("#BFEBBF"),
        text: decor("#DCDCCC"),
        keyword: decor("#F0DFAF"),
        token: decor("#DFAF8F"),
        rule: decor("#93E0E3"),
        string: decor("#CC9393"),
        builtin: decor("#DD6718"),
        unresolved: decor('#FF0000'),
        error: vscode.window.createTextEditorDecorationType({
            borderColor: "red",
            borderStyle: "none none dashed none",
        })
    }

    let syntaxTreeUri = vscode.Uri.parse('fall-tree://foo/bar')
    class TextDocumentProvider implements vscode.TextDocumentContentProvider {
        public eventEmitter = new vscode.EventEmitter<vscode.Uri>()

        public provideTextDocumentContent(uri: vscode.Uri): string {
            return backend.tree()
        }

        get onDidChange(): vscode.Event<vscode.Uri> {
            return this.eventEmitter.event
        }
    }
    let provider = new TextDocumentProvider();

    function highlight() {
        status.hide()
        if (!activeEditor) return
        if (activeEditor.document.languageId != "fall") return

        let decorationSets = {}
        for (let key in decorations) {
            decorationSets[key] = []
        }
        let text = activeEditor.document.getText()
        backend.create(text)
        provider.eventEmitter.fire(syntaxTreeUri)
        let stats = backend.stats()
        const to_ms = (nanos) => `${(nanos / 1000000).toFixed(2)} ms`
        status.text = `lexing: ${to_ms(stats.lexing_time)}`
            + ` parsing: ${to_ms(stats.parsing_time)}`
            + ` reparse: ${stats.reparse_range[1] - stats.reparse_range[0]}`
        status.show()

        for (let [x, y, type] of backend.highlight()) {
            if (!decorationSets[type]) {
                console.log(x, y, type)
                continue
            }

            let px = activeEditor.document.positionAt(x)
            let py = activeEditor.document.positionAt(y)
            decorationSets[type].push(new vscode.Range(px, py))
        }

        for (let type in decorationSets) {
            let deco = decorations[type]
            let ranges = decorationSets[type]
            activeEditor.setDecorations(deco, ranges)
        }
    }

    vscode.window.onDidChangeActiveTextEditor(editor => {
        activeEditor = editor
        highlight()
    }, null, context.subscriptions)
    highlight()


    vscode.workspace.onDidChangeTextDocument(event => {
        if (activeEditor && event.document === activeEditor.document) {
            highlight()
        }
    }, null, context.subscriptions)


    let registration = vscode.workspace.registerTextDocumentContentProvider('fall-tree', provider)
    var showSyntaxTree = vscode.commands.registerCommand('extension.showSyntaxTree', async () => {
        let document = await vscode.workspace.openTextDocument(syntaxTreeUri)
        vscode.window.showTextDocument(document, vscode.ViewColumn.Two, true)
    });
    var semanticSelection = vscode.commands.registerCommand('extension.semanticSelection', () => {
        if (!activeEditor) return
        if (activeEditor.document.languageId != "fall") return

        let doc = activeEditor.document;
        let start = doc.offsetAt(activeEditor.selection.start)
        let end = doc.offsetAt(activeEditor.selection.end)
        let range = backend.extendSelection([start, end])
        if (range == null) return
        let [newStart, newEnd] = range
        let newSelection = new vscode.Selection(doc.positionAt(newStart), doc.positionAt(newEnd))
        activeEditor.selection = newSelection
    });
    context.subscriptions.push(showSyntaxTree, semanticSelection, registration)
    highlight()
}


// this method is called when your extension is deactivated
export function deactivate() {
}