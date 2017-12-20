import * as vscode from 'vscode'
import { log } from 'util'

export function createPlugin(
    backend,
    fileExtension: string,
    disposables: vscode.Disposable[],
    doHighlighting: boolean = false,
    diganosticCollection: vscode.DiagnosticCollection | null = null
) {
    let uris = {
        syntaxTree: vscode.Uri.parse(`fall-${fileExtension}://syntaxtree`),
        metrics: vscode.Uri.parse(`fall-${fileExtension}://metrics`)
    }
    let textDocumentContentProvider = new TextDocumentContentProvider(currentFile, uris)

    let getFile = documentToFile(backend, fileExtension, disposables, (doc) => {
        let emitter = textDocumentContentProvider.eventEmitter
        emitter.fire(uris.syntaxTree)
        emitter.fire(uris.metrics)
        updateActiveEditor()
    })

    function updateActiveEditor() {
        let editor = vscode.window.activeTextEditor
        if (editor == null) return
        let file = currentFile()
        if (file == null) return
        if (doHighlighting) {
            setHighlights(editor, file.highlight())
        }
        if (diganosticCollection != null) {
            diganosticCollection.clear()
            diganosticCollection.set(
                editor.document.uri,
                file.diagnostics()
            )
        }
    }


    function currentFile(): EditorFile | null {
        let editor = vscode.window.activeTextEditor
        if (editor == null) return
        let doc = editor.document
        return getFile(doc)
    }

    vscode.window.onDidChangeActiveTextEditor(updateActiveEditor)
    let cmd = vscode.commands.registerCommand(`fall-${fileExtension}.applyContextAction`, (range, id) => {
        let file = currentFile()
        if (file == null) return
        return file.applyContextAction(range, id)
    })
    disposables.push(cmd)

    return {
        getFile: getFile,
        showSyntaxTree: () => {
            let file = currentFile()
            if (file == null) return
            return openDoc(uris.syntaxTree)
        },
        metrics: () => {
            let file = currentFile()
            if (file == null) return
            return openDoc(uris.metrics)
        },
        extendSelection: () => {
            let editor = vscode.window.activeTextEditor
            let file = currentFile()
            if (editor == null || file == null) return
            let range = file.extendSelection(editor.selection)
            let newSelection = new vscode.Selection(range.start, range.end)
            editor.selection = newSelection
        },
        textDocumentContentProvider: textDocumentContentProvider,
        documentSymbolsProvider: new DocumentSymbolProvider(getFile),
        documentFormattingEditProvider: new DocumentFormattingEditProvider(getFile),
        codeActionProvider: new CodeActionProvider(getFile, fileExtension)
    }
}


export interface FileStructureNode {
    name: string
    range: [number, number]
    children: [FileStructureNode]
}

export interface FallDiagnostic {
    range: [number, number]
    severity: string
    message: string
}

export class EditorFile {
    backend;
    imp;
    doc: vscode.TextDocument;

    constructor(backend, imp, doc: vscode.TextDocument) {
        this.backend = backend
        this.imp = imp
        this.doc = doc
    }

    metrics(): string { return this.call("metrics") }
    syntaxTree(): string { return this.call("syntaxTree") }
    extendSelection(range_: vscode.Range): vscode.Range | null {
        let range = fromVsRange(this.doc, range_)
        let exp = this.call("extendSelection", range)
        if (exp == null) return null
        return toVsRange(this.doc, exp)
    }

    structure(): Array<FileStructureNode> { return this.call("structure") }
    reformat(): Array<vscode.TextEdit> {
        return this.call("reformat").map((op) => {
            return vscode.TextEdit.replace(toVsRange(this.doc, op.delete), op.insert)
        })
    }

    highlight(): Array<[[number, number], string]> { return this.call("highlight") }
    diagnostics(): Array<vscode.Diagnostic> {
        return this.call("diagnostics").map((d) => {
            let range = toVsRange(this.doc, d.range)
            let severity = d.severity == "Error"
                ? vscode.DiagnosticSeverity.Error
                : vscode.DiagnosticSeverity.Warning

            return new vscode.Diagnostic(range, d.message, severity)
        })
    }

    contextActions(range_: vscode.Range): Array<string> {
        let range = fromVsRange(this.doc, range_)
        let result = this.call("contextActions", range)
        return result
    }

    applyContextAction(range_: vscode.Range, id: string) {
        let range = fromVsRange(this.doc, range_)
        let edits = this.call("applyContextAction", range, id)
        let editor = vscode.window.activeTextEditor
        return editor.edit((builder) => {
            for (let op of edits) {
                builder.replace(toVsRange(this.doc, op.delete), op.insert)
            }
        })
    }

    call(method: string, ...args) {
        let result = this.backend[method](this.imp, ...args)
        return result
    }
}

function documentToFile(backend, fileExtension: string, disposables: vscode.Disposable[], onChange) {
    let docs = {}
    function update(doc: vscode.TextDocument, file) {
        let key = doc.uri.toString()
        if (file == null) {
            delete docs[key]
        } else {
            docs[key] = file
        }
        onChange(doc)
    }
    function get(doc: vscode.TextDocument) {
        return docs[doc.uri.toString()]
    }

    function isKnownDoc(doc: vscode.TextDocument) {
        return doc.fileName.endsWith(`.${fileExtension}`)
    }

    vscode.workspace.onDidChangeTextDocument((event: vscode.TextDocumentChangeEvent) => {
        let doc = event.document
        if (!isKnownDoc(event.document)) return
        let tree = get(doc)
        if (event.contentChanges.length == 1 && tree) {
            let edits = event.contentChanges.map((change) => {
                let start = doc.offsetAt(change.range.start)
                return {
                    "delete": [start, start + change.rangeLength],
                    "insert": change.text
                }
            })
            update(doc, backend.edit(tree, edits))
            return
        }
        update(doc, null)
    }, null, disposables)

    vscode.workspace.onDidOpenTextDocument((doc: vscode.TextDocument) => {
        if (!isKnownDoc(doc)) return
        update(doc, backend.parse(doc.getText()))
    }, null, disposables)

    vscode.workspace.onDidCloseTextDocument((doc: vscode.TextDocument) => {
        update(doc, null)
    }, null, disposables)

    return (doc: vscode.TextDocument) => {
        if (!isKnownDoc(doc)) return null

        if (!get(doc)) {
            update(doc, backend.parse(doc.getText()))
        }
        let imp = get(doc)
        return new EditorFile(backend, imp, doc)
    }
}

export class TextDocumentContentProvider implements vscode.TextDocumentContentProvider {
    uris
    currentFile: () => EditorFile | null;
    constructor(currentFile, uris) {
        this.currentFile = currentFile
        this.uris = uris
    }

    public eventEmitter = new vscode.EventEmitter<vscode.Uri>()
    public syntaxTree: string = "Not available"

    public provideTextDocumentContent(uri: vscode.Uri): vscode.ProviderResult<string> {
        let file = this.currentFile()
        if (file == null) return
        if (uri.toString() == this.uris.syntaxTree.toString()) {
            return file.syntaxTree()
        }
        if (uri.toString() == this.uris.metrics.toString()) {
            return file.metrics()
        }
        log(`Bad uri: ${uri}`)
    }

    get onDidChange(): vscode.Event<vscode.Uri> {
        return this.eventEmitter.event
    }
}

export class DocumentSymbolProvider implements vscode.DocumentSymbolProvider {
    getFile: (doc: vscode.TextDocument) => EditorFile | null;
    constructor(getFile) {
        this.getFile = getFile
    }

    provideDocumentSymbols(document: vscode.TextDocument, token: vscode.CancellationToken) {
        let file = this.getFile(document)
        if (file == null) return null
        return file.structure().map((node) => {
            return new vscode.SymbolInformation(
                node.name,
                vscode.SymbolKind.Function,
                toVsRange(document, node.range),
                null,
                null
            )
        })
    }
}

export class DocumentFormattingEditProvider implements vscode.DocumentFormattingEditProvider {
    getFile: (doc: vscode.TextDocument) => EditorFile | null;
    constructor(getFile) { this.getFile = getFile }

    provideDocumentFormattingEdits(
        document: vscode.TextDocument,
        options: vscode.FormattingOptions,
        token: vscode.CancellationToken
    ): vscode.TextEdit[] {
        let file = this.getFile(document)
        if (file == null) return []
        return file.reformat()
    }
}

export class CodeActionProvider implements vscode.CodeActionProvider {
    fileExtension: string
    getFile: (doc: vscode.TextDocument) => EditorFile | null;
    constructor(getFile, fileExtension) {
        this.getFile = getFile
        this.fileExtension = fileExtension
    }

    provideCodeActions(
        document: vscode.TextDocument,
        range: vscode.Range,
        context: vscode.CodeActionContext,
        token: vscode.CancellationToken
    ): vscode.Command[] {
        let file = this.getFile(document)
        if (file == null) return
        let actions = file.contextActions(range)
        return actions.map((id) => {
            return {
                title: id,
                command: `fall-${this.fileExtension}.applyContextAction`,
                arguments: [range, id]
            }
        })
    }
}

const decorations = (() => {
    const decor = (obj) => vscode.window.createTextEditorDecorationType({ color: obj })
    return {
        background: decor("#3F3F3F"),
        error: vscode.window.createTextEditorDecorationType({
            borderColor: "red",
            borderStyle: "none none dashed none",
        }),
        comment: decor("#7F9F7F"),
        string: decor("#CC9393"),
        keyword: decor("#F0DFAF"),
        function: decor("#93E0E3"),
        parameter: decor("#94BFF3"),
        builtin: decor("#DD6718"),
        text: decor("#DCDCCC"),
        attribute: decor("#BFEBBF"),
        literal: decor("#DFAF8F"),
    }
})()

function setHighlights(
    editor: vscode.TextEditor,
    highlihgs: Array<[[number, number], string]>
) {
    let byTag = {}
    for (let tag in decorations) {
        byTag[tag] = []
    }

    for (let [_range, tag] of highlihgs) {
        if (!byTag[tag]) {
            log(`unknown tag ${tag}`)
            continue
        }
        let range = toVsRange(editor.document, _range)
        byTag[tag].push(range)
    }

    for (let tag in byTag) {
        let dec = decorations[tag]
        let ranges = byTag[tag]
        editor.setDecorations(dec, ranges)
    }
}

export function toVsRange(doc: vscode.TextDocument, range: [number, number]): vscode.Range {
    return new vscode.Range(
        doc.positionAt(range[0]),
        doc.positionAt(range[1]),
    )
}

function fromVsRange(doc: vscode.TextDocument, range: vscode.Range): [number, number] {
    return [doc.offsetAt(range.start), doc.offsetAt(range.end)]
}

async function openDoc(uri: vscode.Uri) {
    let document = await vscode.workspace.openTextDocument(uri)
    vscode.window.showTextDocument(document, vscode.ViewColumn.Two, true)
}