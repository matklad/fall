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

let newFile: (string) => any = require('../../native').newFile

var currentFile: {
    highlight: () => [TextRange, string][];

    syntaxTree: () => string;
    structure: () => [FileStructureNode];
    reformat: () => any;
    metrics: () => string;
    diagnostics: () => [{ range: TextRange, severity: string, message: string }];

    extendSelection: (TextRange) => TextRange;
    contextActions: (TextRange) => string[];
    testAtOffset: (number) => number;
    resolveReference: (number) => TextRange;
    findUsages: (number) => TextRange[];

    applyContextAction: (TextRange, string) => any;
    parseTest: (number, callback) => string;
}

export function activate(context: vscode.ExtensionContext) {
    var activeEditor = window.activeTextEditor;
    let syntaxTreeUri = vscode.Uri.parse('fall://syntaxTree')
    let parsedTestUri = vscode.Uri.parse('fall://parsedTest')
    let metricsUri = vscode.Uri.parse('fall://metrics')

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
        error: vscode.window.createTextEditorDecorationType({
            borderColor: "red",
            borderStyle: "none none dashed none",
        })
    }

    let fallDiagnostics = vscode.languages.createDiagnosticCollection("fall")

    class TextDocumentProvider implements vscode.TextDocumentContentProvider {
        public eventEmitter = new vscode.EventEmitter<vscode.Uri>()

        public provideTextDocumentContent(uri: vscode.Uri): vscode.ProviderResult<string> {
            if (uri.authority == 'syntaxTree') {
                return currentFile.syntaxTree()
            } else if (uri.authority == 'parsedTest') {
                return new Promise((resolve, reject) => {
                    currentFile.parseTest(activeTest, (err, value) => {
                        if (err) return reject(err)
                        resolve(value)
                    })
                })
            } else if (uri.authority == "metrics") {
                return currentFile.metrics()
            } else {
                console.log(uri);
            }
        }

        get onDidChange(): vscode.Event<vscode.Uri> {
            return this.eventEmitter.event
        }
    }
    let textDocumentProvider = new TextDocumentProvider();

    var activeTest = null

    class CodeActionProvider implements vscode.CodeActionProvider {
        provideCodeActions(document: TextDocument, _range: Range, context: CodeActionContext, token: CancellationToken): Command[] {
            let range: TextRange = [
                document.offsetAt(_range.start),
                document.offsetAt(_range.end)
            ];
            let test = currentFile.testAtOffset(range[0]);
            if (test != null) {
                return [{
                    title: "Parse test",
                    command: 'extension.applyContextAction',
                    arguments: [-1, test]
                }]
            }

            let actions = currentFile.contextActions(range);
            return actions.map((id) => {
                return {
                    title: id,
                    command: 'extension.applyContextAction',
                    arguments: [range, id]
                }
            })
        }
    }

    function convertRange(document: TextDocument, [start, end]: TextRange): Range {
        return new Range(document.positionAt(start), document.positionAt(end))
    }

    class DocumentSymbolProvider implements vscode.DocumentSymbolProvider {
        provideDocumentSymbols(document: TextDocument, token: CancellationToken) {
            if (!activeEditor) return
            if (activeEditor.document.languageId != "fall") return
            if (document != activeEditor.document) return null
            return currentFile.structure().map((node) => {
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
            let range = currentFile.resolveReference(document.offsetAt(position))
            if (range == null) return null
            return new vscode.Location(document.uri, convertRange(document, range))
        }
    }

    class ReferenceProvider implements vscode.ReferenceProvider {
        provideReferences(document: TextDocument, position: Position, context: vscode.ReferenceContext, token: CancellationToken): vscode.Location[] {
            if (!activeEditor) return
            if (activeEditor.document.languageId != "fall") return
            if (document != activeEditor.document) return null
            let usages = currentFile.findUsages(document.offsetAt(position))
            return usages.map((range) => new vscode.Location(document.uri, convertRange(document, range)))
        }
    }

    class DocumentFormattingEditProvider implements vscode.DocumentFormattingEditProvider {
        provideDocumentFormattingEdits(document: TextDocument, options: vscode.FormattingOptions, token: CancellationToken): TextEdit[] {
            if (!activeEditor) return
            if (activeEditor.document.languageId != "fall") return
            if (document != activeEditor.document) return null
            let edit = currentFile.reformat()
            
            return edit.ops.map((op) => {
                if (op.Insert != null) {
                    let [pos, text] = op.Insert
                    return TextEdit.insert(activeEditor.document.positionAt(pos), text)
                } else if (op.Delete != null) {
                    return TextEdit.delete(convertRange(activeEditor.document, op.Delete))
                }
            })
        }
    }

    function highlight() {
        if (!activeEditor) return
        if (activeEditor.document.languageId != "fall") return
        currentFile = newFile(activeEditor.document.getText())

        let decorationSets = {}
        for (let key in decorations) {
            decorationSets[key] = []
        }
        let text = activeEditor.document.getText()
        textDocumentProvider.eventEmitter.fire(syntaxTreeUri)

        for (let [[x, y], type] of currentFile.highlight()) {
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
            currentFile.diagnostics().map((d) => {
                let range = new Range(
                    activeEditor.document.positionAt(d.range[0]),
                    activeEditor.document.positionAt(d.range[1]),
                )
                
                let severity = d.severity == "Error"
                    ? vscode.DiagnosticSeverity.Error
                    : vscode.DiagnosticSeverity.Warning

                return new vscode.Diagnostic(range, d.message, severity)
            })
        )
        textDocumentProvider.eventEmitter.fire(metricsUri)
        
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
        textDocumentProvider.eventEmitter.fire(parsedTestUri)
    })

    let providers = [
        vscode.workspace.registerTextDocumentContentProvider('fall', textDocumentProvider),
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
        vscode.commands.registerCommand('extension.showMetrics', async () => {
            let document = await vscode.workspace.openTextDocument(metricsUri)
            vscode.window.showTextDocument(document, vscode.ViewColumn.Two, true)
        }),
        vscode.commands.registerCommand('extension.semanticSelection', () => {
            if (!activeEditor) return
            if (activeEditor.document.languageId != "fall") return

            let doc = activeEditor.document;
            let start = doc.offsetAt(activeEditor.selection.start)
            let end = doc.offsetAt(activeEditor.selection.end)
            let range = currentFile.extendSelection([start, end])
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
                textDocumentProvider.eventEmitter.fire(parsedTestUri)
                let document = await vscode.workspace.openTextDocument(parsedTestUri)
                vscode.window.showTextDocument(document, vscode.ViewColumn.Two, true)
                return
            }

            let edit = currentFile.applyContextAction(offset, id);
            return activeEditor.edit((builder) => {
                for (let op of edit.ops) {
                    if (op.Insert != null) {
                        let [pos, text] = op.Insert
                        builder.insert(activeEditor.document.positionAt(pos), text)
                    } else if (op.Delete != null) {
                        builder.delete(convertRange(activeEditor.document, op.Delete))
                    }
                }
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