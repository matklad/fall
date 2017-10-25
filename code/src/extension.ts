'use strict';
import * as vscode from 'vscode';
import {
    window, DecorationRenderOptions, StatusBarAlignment, TextDocument,
    Range, Position, CodeActionContext, CancellationToken, Command, SymbolInformation,
    TextEdit
} from 'vscode';

interface FileStructureNode {
    name: string,
    range: TextRange,
    children: [FileStructureNode]
}

type TextRange = [number, number]

var backend = (() => {
    var native = require('../../native')
    return {
        treeAsText: (): string => native.tree_as_text(),
        performaceCounters: (): {lexing_time: number, parsing_time: number, reparsed_region: TextRange} => {
            return native.performance_counters()
        },
        highlight: (): [TextRange, string][] => native.highlight(),
        structure: (): [FileStructureNode] => native.structure(),
        extendSelection: (range: TextRange) => native.extend_selection(range),
        create: (text) => native.file_create(text),
        findContextActions: (offset: number): string[] => native.file_find_context_actions(offset),
        applyContextAction: (offset: number, id: string) => native.file_apply_context_action(offset, id),
        diagnostics: (): [{ range: TextRange, text: string, severity: string }] => native.file_diagnostics(),
        resolveReference: (offset: number): TextRange => native.file_resolve_reference(offset),
        findUsages: (offset: number): TextRange[] => native.file_find_usages(offset),
        reformat: (): [number, number, string] => native.file_reformat(),
        findTestAtOffset: (offset: number): number => native.file_find_test_at_offset(offset),
        //parse_test: (testId: number): string => native.file_parse_test(testId),
        parse_test: (testId: number, callback): string => native.parse_test(testId, callback),
    }
})()

export function activate(context: vscode.ExtensionContext) {
    var status = window.createStatusBarItem(StatusBarAlignment.Left)
    var activeEditor = window.activeTextEditor;
    let syntaxTreeUri = vscode.Uri.parse('fall-tree://syntaxTree')
    let parsedTestUri = vscode.Uri.parse('fall-test://parsedTest')

    const decor = (obj) => vscode.window.createTextEditorDecorationType({ color: obj })
    const decorations = {
        background: decor("#3F3F3F"),
        meta: decor("#BFEBBF"),
        text: decor("#DCDCCC"),
        keyword: decor("#F0DFAF"),
        token: decor("#DFAF8F"),
        rule: decor("#93E0E3"),
        value_parameter: decor("#94BFF3"),
        string: decor("#CC9393"),
        builtin: decor("#DD6718"),
        comment: decor("#7F9F7F"),
        unresolved: decor('#FF0000'),
        error: vscode.window.createTextEditorDecorationType({
            borderColor: "red",
            borderStyle: "none none dashed none",
        })
    }

    let fallDiagnostics = vscode.languages.createDiagnosticCollection("fall")

    class TreeTextDocumentProvider implements vscode.TextDocumentContentProvider {
        public eventEmitter = new vscode.EventEmitter<vscode.Uri>()

        public provideTextDocumentContent(uri: vscode.Uri): string {
            return backend.treeAsText()
        }

        get onDidChange(): vscode.Event<vscode.Uri> {
            return this.eventEmitter.event
        }
    }
    let treeTextDocumentProvider = new TreeTextDocumentProvider();

    var activeTest = null
    class TestTextDocumentProvider implements vscode.TextDocumentContentProvider {
        public eventEmitter = new vscode.EventEmitter<vscode.Uri>()

        public provideTextDocumentContent(uri: vscode.Uri): Promise<string> {
            return new Promise((resolve, reject) => {
                backend.parse_test(activeTest, (err, value) => {
                    if (err) return reject(err) 
                    resolve(value)
                })
            })
        }

        get onDidChange(): vscode.Event<vscode.Uri> {
            return this.eventEmitter.event
        }
    }
    let testTextDocumentProvider = new TestTextDocumentProvider();

    class CodeActionProvider implements vscode.CodeActionProvider {
        provideCodeActions(document: TextDocument, range: Range, context: CodeActionContext, token: CancellationToken): Command[] {
            let offset = document.offsetAt(range.start);
            let test = backend.findTestAtOffset(offset);
            if (test != null) {
                return [{
                    title: "Parse test",
                    command: 'extension.applyContextAction',
                    arguments: [-1, test]
                }]
            }

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

    function convertRange(document: TextDocument, range: TextRange): Range {
        return new Range(document.positionAt(range[0]), document.positionAt(range[1]))
    }

    class DocumentSymbolProvider implements vscode.DocumentSymbolProvider {
        provideDocumentSymbols(document: TextDocument, token: CancellationToken) {
            if (!activeEditor) return
            if (activeEditor.document.languageId != "fall") return
            if (document != activeEditor.document) return null
            return backend.structure().map((node) => {
                return new SymbolInformation(
                    node.name,
                    vscode.SymbolKind.Function,
                    convertRange(document, node.range),
                    null,
                    null
                )
            })
        }
    }

    class DefinitionProvider implements vscode.DefinitionProvider {
        provideDefinition(document: TextDocument, position: Position, token: CancellationToken): vscode.Location {
            if (!activeEditor) return
            if (activeEditor.document.languageId != "fall") return
            if (document != activeEditor.document) return null
            let range = backend.resolveReference(document.offsetAt(position))
            if (range == null) return null
            return new vscode.Location(document.uri, convertRange(document, range))
        }
    }

    class ReferenceProvider implements vscode.ReferenceProvider {
        provideReferences(document: TextDocument, position: Position, context: vscode.ReferenceContext, token: CancellationToken): vscode.Location[] {
            if (!activeEditor) return
            if (activeEditor.document.languageId != "fall") return
            if (document != activeEditor.document) return null
            let usages = backend.findUsages(document.offsetAt(position))
            return usages.map((range) => new vscode.Location(document.uri, convertRange(document, range)))
        }
    }

    class DocumentFormattingEditProvider implements vscode.DocumentFormattingEditProvider {
        provideDocumentFormattingEdits(document: TextDocument, options: vscode.FormattingOptions, token: CancellationToken): TextEdit[] {
            if (!activeEditor) return
            if (activeEditor.document.languageId != "fall") return
            if (document != activeEditor.document) return null
            let rawEdit = backend.reformat()
            return [new TextEdit(convertRange(document, [rawEdit[0], rawEdit[1]]), rawEdit[2])]
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
        treeTextDocumentProvider.eventEmitter.fire(syntaxTreeUri)
        let stats = backend.performaceCounters()
        const to_ms = (nanos) => `${(nanos / 1000000).toFixed(2)} ms`
        status.text = `lexing: ${to_ms(stats.lexing_time)}`
            + ` parsing: ${to_ms(stats.parsing_time)}`
            + ` reparse: ${stats.reparsed_region[1] - stats.reparsed_region[0]}`
        status.show()

        for (let [[x, y], type] of backend.highlight()) {
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

        fallDiagnostics.clear()
        fallDiagnostics.set(
            activeEditor.document.uri,
            backend.diagnostics().map((d) => {
                let range = new Range(
                    activeEditor.document.positionAt(d.range[0]),
                    activeEditor.document.positionAt(d.range[1]),
                )
                let severity = d.severity == "error"
                    ? vscode.DiagnosticSeverity.Error
                    : vscode.DiagnosticSeverity.Warning

                return new vscode.Diagnostic(range, d.text, severity)
            })
        )
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

    vscode.workspace.onDidSaveTextDocument(event => {
        testTextDocumentProvider.eventEmitter.fire(parsedTestUri)
    })

    let providers = [
        vscode.workspace.registerTextDocumentContentProvider('fall-tree', treeTextDocumentProvider),
        vscode.workspace.registerTextDocumentContentProvider('fall-test', testTextDocumentProvider),
        vscode.languages.registerCodeActionsProvider('fall', new CodeActionProvider()),
        vscode.languages.registerDocumentSymbolProvider('fall', new DocumentSymbolProvider()),
        vscode.languages.registerDefinitionProvider('fall', new DefinitionProvider()),
        vscode.languages.registerReferenceProvider('fall', new ReferenceProvider()),
        vscode.languages.registerDocumentFormattingEditProvider('fall', new DocumentFormattingEditProvider()),
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
        vscode.commands.registerCommand('extension.applyContextAction', async (offset, id) => {
            if (!activeEditor) return
            if (activeEditor.document.languageId != "fall") return
            if (offset == -1) {
                activeTest = id
                testTextDocumentProvider.eventEmitter.fire(parsedTestUri)
                let document = await vscode.workspace.openTextDocument(parsedTestUri)
                vscode.window.showTextDocument(document, vscode.ViewColumn.Two, true)
                return
            }

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
    context.subscriptions.push(fallDiagnostics)
    highlight()
}


// this method is called when your extension is deactivated
export function deactivate() {
}