import * as vscode from 'vscode'

const native = require("../../native")

export let backend = {
    status() {
         return native.status()
    },

}

export class LangSupport {
    static forExtension(ext: string): LangSupport | null {
        return new LangSupport()
    }

    showSyntaxTree(): string {
        return "Hello"
    }
}