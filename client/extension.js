const path = require('path');
const { LanguageClient } = require('vscode-languageclient/node');

let client;

function activate(context) {
    const serverExe = path.join(context.extensionPath,'tom-lsp', 'target', 'debug', 'tom-lsp.exe');

    console.log("Launching TOM LSP at", serverExe);

    const serverOptions = {
        run: { command: serverExe },
        debug: { command: serverExe }
    };

    const clientOptions = {
        documentSelector: [{ scheme: 'file', language: 'tom' }]
    };

    client = new LanguageClient(
        'tomLanguageServer',
        'TOM Language Server',
        serverOptions,
        clientOptions
    );

    exports.activate = activate;
    client.start();
}

function deactivate() {
    if (!client) return undefined;
    return client.stop();
}

module.exports = { activate, deactivate };
