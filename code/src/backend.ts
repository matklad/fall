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

interface FileStructureNode {
    name: string,
    range: [number, number],
    children: [FileStructureNode]
}

export class VsFile {
    impl;
    constructor(impl) {
        this.impl = impl
    }

    edit(edits: Array<{insert: string, delete: [number, number]}>): VsFile {
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

    metrics(): string {
        return this.impl.metrics()
    }
}