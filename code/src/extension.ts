'use strict';
import * as vscode from 'vscode';
import {
    window, DecorationRenderOptions, StatusBarAlignment, TextDocument,
    Range, CodeActionContext, CancellationToken, Command, SymbolInformation
} from 'vscode';

interface FileStructureNode {
    name: string,
    range: [number, number],
    children: [FileStructureNode]
}

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
        tree: (): string => native.file_tree(),
        findContextActions: (offset: number): [string] => native.file_find_context_actions(offset),
        applyContextAction: (offset: number, id: string) => native.file_apply_context_action(offset, id),
        fileStructure: (): [FileStructureNode] => native.file_structure(),
    }
})()

export function activate(context: vscode.ExtensionContext) {
    var status = window.createStatusBarItem(StatusBarAlignment.Left)
    var activeEditor = window.activeTextEditor;
    let syntaxTreeUri = vscode.Uri.parse('fall-tree://foo/bar')

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
        comment: decor("#7F9F7F"),
        unresolved: decor('#FF0000'),
        error: vscode.window.createTextEditorDecorationType({
            borderColor: "red",
            borderStyle: "none none dashed none",
        })
    }

    class TextDocumentProvider implements vscode.TextDocumentContentProvider {
        public eventEmitter = new vscode.EventEmitter<vscode.Uri>()

        public provideTextDocumentContent(uri: vscode.Uri): string {
            return backend.tree()
        }

        get onDidChange(): vscode.Event<vscode.Uri> {
            return this.eventEmitter.event
        }
    }
    let textDocumentProvider = new TextDocumentProvider();

    class CodeActionProvider implements vscode.CodeActionProvider {
        provideCodeActions(document: TextDocument, range: Range, context: CodeActionContext, token: CancellationToken): Command[] {
            let offset = document.offsetAt(range.start);
            let actions = backend.findContextActions(offset);
            return actions.map((id) => {
                return {
                    title: id,
                    command: 'extension.applyContextAction',
                    arguments: [offset, id]
                }
            })
        }
    }

    class DocumentSymbolProvider implements vscode.DocumentSymbolProvider {
        provideDocumentSymbols(document: TextDocument, token: CancellationToken) {
            console.log("A");
            if (!activeEditor) return
            if (activeEditor.document.languageId != "fall") return
            if (document != activeEditor.document) return null
            let offsetToPostion = (x) => document.positionAt(x)
            return backend.fileStructure().map((node) => {
                return new SymbolInformation(
                    node.name,
                    vscode.SymbolKind.Function,
                    new Range(
                        offsetToPostion(node.range[0]),
                        offsetToPostion(node.range[1])
                    ),
                    null,
                    null
                )
            })
        }

    }

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
        textDocumentProvider.eventEmitter.fire(syntaxTreeUri)
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


    let providers = [
        vscode.workspace.registerTextDocumentContentProvider('fall-tree', textDocumentProvider),
        vscode.languages.registerCodeActionsProvider('fall', new CodeActionProvider()),
        vscode.languages.registerDocumentSymbolProvider('fall', new DocumentSymbolProvider())
    ]

    let commands = [
        vscode.commands.registerCommand('extension.showSyntaxTree', async () => {
            let document = await vscode.workspace.openTextDocument(syntaxTreeUri)
            vscode.window.showTextDocument(document, vscode.ViewColumn.Two, true)
        }),
        vscode.commands.registerCommand('extension.semanticSelection', () => {
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
        }),
        vscode.commands.registerCommand('extension.applyContextAction', (offset, id) => {
            if (!activeEditor) return
            if (activeEditor.document.languageId != "fall") return

            let edit = backend.applyContextAction(offset, id);

            let range = new Range(
                activeEditor.document.positionAt(edit[0]),
                activeEditor.document.positionAt(edit[1]),
            )
            activeEditor.edit((bulder) => {
                bulder.replace(range, edit[2])
            })
        })
    ]
    context.subscriptions.push(...commands)
    context.subscriptions.push(...providers)
    highlight()
}


// this method is called when your extension is deactivated
export function deactivate() {
}