'use strict';

import * as vscode from 'vscode';
import commands from './commands';

export function activate(context: vscode.ExtensionContext) {
    console.log("activating extension");
    for (let name in commands) {
        let action = commands[name]
        vscode.commands.registerCommand("fall." + name, action)
    }
}

export function deactivate() {}