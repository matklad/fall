import * as vscode from 'vscode'
import { type } from 'os';

const native = require("../../native")

export let backend = {
    status() {
         return native.status()
    },
}

export class LangSupport {
    sup;
    constructor(sup) {
        this.sup = sup
    }

    static forExtension(ext: string): LangSupport | null {
        let sup = native.supportForExtension(ext)
        if (sup == null) return null
        return new LangSupport(sup)
    }

    parse(text: string): VsFile {
        return this.sup.parse(text)
    }
}

export type VsFile = {
    syntaxTree(): string
    highlight(): Array<[[number, number], string]>
}