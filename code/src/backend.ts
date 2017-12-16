import * as vscode from 'vscode'

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
        let sup = native.support_for_extension(ext)
        if (sup == null) return null
        return new LangSupport(sup)
    }

    showSyntaxTree(text: string): string {
        return this.sup.syntax_tree(text)
    }
}