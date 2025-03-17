import * as vscode from 'vscode';
import * as fs from 'fs';
import * as cp from 'child_process';
import * as path from 'path';

export function activate(context: vscode.ExtensionContext) {
    console.log('Extension "ia-agent" activée');
    const extensionUri = context.extensionUri;

    // Lancer le serveur IA local (server.js est dans src/)
    const serverPath = path.join(context.extensionPath, 'src', 'server.js');
    const serverProcess = cp.spawn('node', [serverPath], { stdio: 'inherit' });
    context.subscriptions.push({
        dispose: () => serverProcess.kill()
    });

    // Commande "Hello World"
    const helloWorldDisposable = vscode.commands.registerCommand('ia-agent.helloWorld', () => {
        vscode.window.showInformationMessage('Hello World from ia_agent!');
    });

    // Commande pour ouvrir la fenêtre de chat (panel principal)
    const openChatDisposable = vscode.commands.registerCommand('ia-agent.openChat', () => {
        const panel = vscode.window.createWebviewPanel(
            'iaAgentChat',
            'Chat with IA Agent',
            vscode.ViewColumn.One,
            { enableScripts: true }
        );
        panel.webview.html = getHtmlContentFromFile(extensionUri, 'chat.html', panel.webview);

        panel.webview.onDidReceiveMessage(async (message) => {
            if (message.command === 'send') {
                const userInput = message.text;
                const prompt = userInput;
                try {
                    const response = await fetch('http://localhost:8000/chat', {
                        method: 'POST',
                        headers: { 'Content-Type': 'application/json' },
                        body: JSON.stringify({ text: prompt })
                    });
                    if (!response.ok) {
                        throw new Error(`Erreur HTTP : ${response.status}`);
                    }
                    const data = await response.json() as { response: string };
                    panel.webview.postMessage({ command: 'response', text: data.response });
                } catch (error: any) {
                    panel.webview.postMessage({ command: 'response', text: 'Error: ' + (error instanceof Error ? error.message : String(error)) });
                }
            }
        });
    });

    // Commande pour ouvrir le panneau latéral (sidebar)
    const openSidebarDisposable = vscode.commands.registerCommand('ia-agent.openSidebar', () => {
        vscode.commands.executeCommand('workbench.view.iaAgent');
    });

    context.subscriptions.push(helloWorldDisposable, openChatDisposable, openSidebarDisposable);

    // Enregistrement du WebviewViewProvider pour la vue "iaAgentView"
    const viewProvider = new IAAgentViewProvider(extensionUri);
    context.subscriptions.push(
        vscode.window.registerWebviewViewProvider(IAAgentViewProvider.viewType, viewProvider)
    );
    console.log('IAAgentViewProvider enregistré');
}

export function deactivate() {}

class IAAgentViewProvider implements vscode.WebviewViewProvider {
    public static readonly viewType = 'iaAgentView';
    private _view?: vscode.WebviewView;

    constructor(private readonly extensionUri: vscode.Uri) {}

    public resolveWebviewView(webviewView: vscode.WebviewView, _context: vscode.WebviewViewResolveContext, _token: vscode.CancellationToken) {
        this._view = webviewView;
        webviewView.webview.options = {
            enableScripts: true,
            localResourceRoots: [vscode.Uri.joinPath(this.extensionUri, 'resources', 'views')]
        };
        // Ici, on charge le fichier chat.html dans la sidebar.
        webviewView.webview.html = getHtmlContentFromFile(this.extensionUri, 'chat.html', webviewView.webview);
        console.log('IAAgentViewProvider: resolveWebviewView exécuté');
    }
}

function getHtmlContentFromFile(extensionUri: vscode.Uri, fileName: string, webview: vscode.Webview): string {
    const filePath = vscode.Uri.joinPath(extensionUri, 'resources', 'views', fileName);
    let html = fs.readFileSync(filePath.fsPath, 'utf8');
    // Remplacer ${webview.cspSource} par la valeur correcte dans le HTML
    html = html.replace(/\${webview\.cspSource}/g, webview.cspSource);
    return html;
}
