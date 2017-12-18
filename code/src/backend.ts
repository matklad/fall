import * as vscode from 'vscode'
import { reportDuration } from './profile'
import { type } from 'os'
import { log } from 'util';

const native = require("../../native")

export class LangSupport {
    impl;
    private constructor(impl) {
        this.impl = impl
    }

    static forExtension(ext: string): LangSupport | null {
        let impl = native.supportForExtension(ext)
        if (impl == null) return null
        return new LangSupport(impl)
    }

    parse(text: string): VsFile {
        return reportDuration("parse", () => new VsFile(this.impl.parse(text)))
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

export interface Edit {
    insert: string
    delete: [number, number]
}

export class VsFile {
    impl;
    constructor(impl) {
        this.impl = impl
    }

    edit(edits: Array<Edit>): VsFile {
        let impl = reportDuration("reparse", () => this.impl.edit(edits))
        return new VsFile(impl)
    }

    syntaxTree(): string {
        return reportDuration("syntaxTree", () => this.impl.syntaxTree())
    }

    structure(): Array<FileStructureNode> {
        return this.impl.structure()
    }

    highlight(): Array<[[number, number], string]> {
        return reportDuration("highlight", () => this.impl.highlight())
    }

    diagnostics(): Array<FallDiagnostic> {
        return reportDuration("diagnostics", () => this.impl.diagnostics())
    }

    extendSelection(range: [number, number]): [number, number] | null {
        return this.impl.extendSelection(range)
    }

    contextActions(range: [number, number]): Array<string> {
        return this.impl.contextActions(range)
    }

    applyContextAction(range: [number, number], id: string): Array<Edit> {
        return this.impl.applyContextAction(range, id)
    }

    reformat(): Array<Edit> {
        return this.impl.reformat()
    }

    metrics(): string {
        return this.impl.metrics()
    }
}