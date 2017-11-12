'use strict';
import * as vscode from 'vscode';
import {
    window, DecorationRenderOptions, StatusBarAlignment, TextDocument,
    Range, Position, CodeActionContext, CancellationToken, Command, SymbolInformation,
    TextEdit
} from 'vscode';


type TextRange = [number, number]
function TextRange2Range(document: TextDocument, [start, end]: TextRange): Range {
    return new Range(document.positionAt(start), document.positionAt(end))
}
function TextRange2Location(document: TextDocument, range: TextRange): vscode.Location {
    return new vscode.Location(document.uri, TextRange2Range(document, range))
}
function Range2TextRange(document: TextDocument, range: Range): TextRange {
    return [document.offsetAt(range.start), document.offsetAt(range.end)]
}


interface FileStructureNode {
    name: string,
    range: TextRange,
    children: [FileStructureNode]
}
type File = {
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

const uris = {
    syntaxTree: vscode.Uri.parse('fall://syntaxTree'),
    parsedTest: vscode.Uri.parse('fall://parsedTest'),
    metrics: vscode.Uri.parse('fall://metrics')
}

const decorations = (() => {
    const decor = (obj) => vscode.window.createTextEditorDecorationType({ color: obj })
    return {
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
})()

const fallDiagnostics = vscode.languages.createDiagnosticCollection("fall")



var activeEditor: vscode.TextEditor = null
var currentFile: File = null
var currentTest = null

function isFallDocument(doc: TextDocument) {
    return (activeEditor != null && activeEditor.document == doc)
}


let newFile: (string) => File = require('../../native').newFile
function switchEditor(editor) {
    if (editor != null && editor.document.languageId == "fall") {
        activeEditor = editor
        currentFile = newFile(editor.document.getText())
    } else {
        activeEditor = null
        currentFile = null
    }
}

class TextDocumentProvider implements vscode.TextDocumentContentProvider {
    public eventEmitter = new vscode.EventEmitter<vscode.Uri>()

    public provideTextDocumentContent(uri: vscode.Uri): vscode.ProviderResult<string> {
        if (uri.authority == 'syntaxTree') {
            return currentFile.syntaxTree()
        } else if (uri.authority == 'parsedTest') {
            return new Promise((resolve, reject) => {
                currentFile.parseTest(currentTest, (err, value) => {
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
const textDocumentProvider = new TextDocumentProvider()

class CodeActionProvider implements vscode.CodeActionProvider {
    provideCodeActions(document: TextDocument, _range: Range, context: CodeActionContext, token: CancellationToken): Command[] {
        if (!isFallDocument(document)) return
        let range = Range2TextRange(document, _range);
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

class DocumentSymbolProvider implements vscode.DocumentSymbolProvider {
    provideDocumentSymbols(document: TextDocument, token: CancellationToken) {
        if (!isFallDocument(document)) return
        return currentFile.structure().map((node) => {
            return new SymbolInformation(
                node.name,
                vscode.SymbolKind.Function,
                TextRange2Range(document, node.range),
                null,
                null
            )
        })
    }
}

class DefinitionProvider implements vscode.DefinitionProvider {
    provideDefinition(document: TextDocument, position: Position, token: CancellationToken): vscode.Location {
        if (!isFallDocument(document)) return
        let range = currentFile.resolveReference(document.offsetAt(position))
        if (range == null) return null
        return TextRange2Location(document, range)
    }
}

class ReferenceProvider implements vscode.ReferenceProvider {
    provideReferences(document: TextDocument, position: Position, context: vscode.ReferenceContext, token: CancellationToken): vscode.Location[] {
        if (!isFallDocument(document)) return
        return currentFile.findUsages(document.offsetAt(position))
            .map((range) => TextRange2Location(document, range))
    }
}

class DocumentFormattingEditProvider implements vscode.DocumentFormattingEditProvider {
    provideDocumentFormattingEdits(document: TextDocument, options: vscode.FormattingOptions, token: CancellationToken): TextEdit[] {
        if (!isFallDocument(document)) return
        return currentFile.reformat().map((op) => {
            return TextEdit.replace(TextRange2Range(activeEditor.document, op.delete), op.insert)
        })
    }
}

function highlight() {
    if (!activeEditor) return
    

    let decorationSets = {}
    for (let key in decorations) {
        decorationSets[key] = []
    }
    textDocumentProvider.eventEmitter.fire(uris.syntaxTree)

    for (let [_range, type] of currentFile.highlight()) {
        let range = TextRange2Range(activeEditor.document, _range)
        if (!decorationSets[type]) {
            console.log(type)
            continue
        }
        decorationSets[type].push(range)
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
            let range = TextRange2Range(activeEditor.document, d.range)

            let severity = d.severity == "Error"
                ? vscode.DiagnosticSeverity.Error
                : vscode.DiagnosticSeverity.Warning

            return new vscode.Diagnostic(range, d.message, severity)
        })
    )
    textDocumentProvider.eventEmitter.fire(uris.metrics)
}


export function activate(context: vscode.ExtensionContext) {
    switchEditor(window.activeTextEditor)

    vscode.window.onDidChangeActiveTextEditor(editor => {
        switchEditor(editor)
        highlight()
    }, null, context.subscriptions)

    vscode.workspace.onDidChangeTextDocument(event => {        
        if (isFallDocument(event.document)) {
            currentFile = newFile(event.document.getText())
            highlight()
        }
    }, null, context.subscriptions)

    vscode.workspace.onDidSaveTextDocument(event => {
        textDocumentProvider.eventEmitter.fire(uris.parsedTest)
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
            let document = await vscode.workspace.openTextDocument(uris.syntaxTree)
            vscode.window.showTextDocument(document, vscode.ViewColumn.Two, true)
        }),
        vscode.commands.registerCommand('extension.showMetrics', async () => {
            let document = await vscode.workspace.openTextDocument(uris.metrics)
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
                currentTest = id
                textDocumentProvider.eventEmitter.fire(uris.parsedTest)
                let document = await vscode.workspace.openTextDocument(uris.parsedTest)
                vscode.window.showTextDocument(document, vscode.ViewColumn.Two, true)
                return
            }

            return activeEditor.edit((builder) => {
                for (let op of currentFile.applyContextAction(offset, id)) {
                    builder.replace(TextRange2Range(activeEditor.document, op.delete), op.insert)
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