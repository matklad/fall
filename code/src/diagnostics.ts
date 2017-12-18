import * as vscode from 'vscode'
import { FallDiagnostic} from './backend'
import { toVsRange } from './range'

const fallDiagnostics = vscode.languages.createDiagnosticCollection("fall")

export function setDiagnostics(
    document: vscode.TextDocument,
    diagnostics: Array<FallDiagnostic>
) {
    fallDiagnostics.clear()
    fallDiagnostics.set(
        document.uri,
        diagnostics.map((d) => {
            let range = toVsRange(document, d.range)
            let severity = d.severity == "Error"
                ? vscode.DiagnosticSeverity.Error
                : vscode.DiagnosticSeverity.Warning

            return new vscode.Diagnostic(range, d.message, severity)
        })
    )
}
