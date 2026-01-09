import * as path from 'path';
import * as fs from 'fs';
import * as vscode from 'vscode';
import {
    LanguageClient,
    LanguageClientOptions,
    ServerOptions,
    Executable,
} from 'vscode-languageclient/node';

let client: LanguageClient;

function findServerPath(context: vscode.ExtensionContext): string | undefined {
    const config = vscode.workspace.getConfiguration('sayo-asm');
    let serverPath = config.get<string>('languageServer.path');
    
    // Priority 1: User-configured path (for development)
    if (serverPath && fs.existsSync(serverPath)) {
        console.log(`Using configured LSP server: ${serverPath}`);
        return serverPath;
    }
    
    // Priority 2: Bundled executable in extension
    const bundledPath = path.join(context.extensionPath, 'bin', 'sayo-lsp.exe');
    if (fs.existsSync(bundledPath)) {
        console.log(`Using bundled LSP server: ${bundledPath}`);
        return bundledPath;
    }
    
    // Priority 3: Development build in workspace
    const workspaceFolders = vscode.workspace.workspaceFolders;
    if (workspaceFolders && workspaceFolders.length > 0) {
        const workspaceRoot = workspaceFolders[0].uri.fsPath;
        const devPaths = [
            path.join(workspaceRoot, 'target', 'release', 'sayo-lsp.exe'),
            path.join(workspaceRoot, 'target', 'debug', 'sayo-lsp.exe')
        ];
        
        for (const devPath of devPaths) {
            if (fs.existsSync(devPath)) {
                console.log(`Using development LSP server: ${devPath}`);
                return devPath;
            }
        }
    }
    
    return undefined;
}

export function activate(context: vscode.ExtensionContext) {
    console.log('Sayo Assembly extension activated');

    const serverPath = findServerPath(context);
    
    if (!serverPath) {
        vscode.window.showErrorMessage(
            'Sayo LSP server not found. Please install the extension properly or configure "sayo-asm.languageServer.path"'
        );
        return;
    }

    const run: Executable = {
        command: serverPath,
        options: {
            // This allows the server to be stopped properly
            detached: false
        }
    };

    const serverOptions: ServerOptions = {
        run,
        debug: run,
    };

    const config = vscode.workspace.getConfiguration('sayo-asm');
    const clientOptions: LanguageClientOptions = {
        documentSelector: [{ scheme: 'file', language: 'sayo-asm' }],
        synchronize: {
            fileEvents: vscode.workspace.createFileSystemWatcher('**/*.{s,asm}'),
        },
        outputChannelName: 'Sayo Assembly Language Server',
        traceOutputChannel: config.get('trace.server') !== 'off' 
            ? vscode.window.createOutputChannel('Sayo Assembly Language Server Trace')
            : undefined
    };

    client = new LanguageClient(
        'sayo-asm',
        'Sayo Assembly Language Server',
        serverOptions,
        clientOptions
    );

    client.start();
    
    console.log('Sayo LSP client started');
}

export function deactivate(): Thenable<void> | undefined {
    if (!client) {
        return undefined;
    }
    return client.stop();
}
