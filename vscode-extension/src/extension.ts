import * as path from 'path';
import * as vscode from 'vscode';
import {
    LanguageClient,
    LanguageClientOptions,
    ServerOptions,
    Executable,
} from 'vscode-languageclient/node';

let client: LanguageClient;

export function activate(context: vscode.ExtensionContext) {
    console.log('Sayo Assembly extension activated');

    // Get LSP server path from configuration or use default
    const config = vscode.workspace.getConfiguration('sayo-asm');
    let serverPath = config.get<string>('languageServer.path');
    
    if (!serverPath) {
        // Try to find the binary in the workspace
        const workspaceFolders = vscode.workspace.workspaceFolders;
        if (workspaceFolders && workspaceFolders.length > 0) {
            const workspaceRoot = workspaceFolders[0].uri.fsPath;
            serverPath = path.join(workspaceRoot, 'target', 'debug', 'sayo-lsp.exe');
        }
    }

    if (!serverPath) {
        vscode.window.showErrorMessage('Sayo LSP server path not configured');
        return;
    }

    const run: Executable = {
        command: serverPath,
    };

    const serverOptions: ServerOptions = {
        run,
        debug: run,
    };

    const clientOptions: LanguageClientOptions = {
        documentSelector: [{ scheme: 'file', language: 'sayo-asm' }],
        synchronize: {
            fileEvents: vscode.workspace.createFileSystemWatcher('**/*.{s,asm}'),
        },
    };

    client = new LanguageClient(
        'sayo-asm',
        'Sayo Assembly Language Server',
        serverOptions,
        clientOptions
    );

    client.start();
}

export function deactivate(): Thenable<void> | undefined {
    if (!client) {
        return undefined;
    }
    return client.stop();
}
