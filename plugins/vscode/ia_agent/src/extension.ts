import * as vscode from 'vscode';
import * as fs from 'fs';
import * as cp from 'child_process';
import * as path from 'path';

export function activate(context: vscode.ExtensionContext) {
    const extensionUri = context.extensionUri;

    // Lancer le serveur IA local
    const serverProcess = cp.spawn('node', [path.join(context.extensionPath, 'src', 'server.js')], { stdio: 'inherit' });
    context.subscriptions.push({ dispose: () => serverProcess.kill() });

    // Enregistrer les commandes
    context.subscriptions.push(
        vscode.commands.registerCommand('ia-agent.helloWorld', () =>
            vscode.window.showInformationMessage('Hello World from ia_agent!')
        ),
        vscode.commands.registerCommand('ia-agent.openChat', () => {
            const panel = vscode.window.createWebviewPanel('iaAgentChat', 'Chat with IA Agent', vscode.ViewColumn.One, {
                enableScripts: true,
                localResourceRoots: [vscode.Uri.joinPath(extensionUri, 'resources')]
            });
            panel.webview.html = getHtmlContentFromFile(extensionUri, 'chat.html', panel.webview);
            panel.webview.onDidReceiveMessage(async message => {
                if (message.command === 'send') {
                    try {
                        const response = await fetch('http://localhost:8000/chat', {
                            method: 'POST',
                            headers: { 'Content-Type': 'application/json' },
                            body: JSON.stringify({ text: message.text })
                        });
                        const data = await response.json() as { response: string };
                        panel.webview.postMessage({ command: 'response', text: data.response });
                    } catch (error: any) {
                        panel.webview.postMessage({ command: 'response', text: 'Error: ' + error.message });
                    }
                }
            });
        }),
        vscode.commands.registerCommand('ia-agent.openSidebar', () =>
            vscode.commands.executeCommand('workbench.view.iaAgent')
        )
    );

    // Enregistrer le WebviewViewProvider
    context.subscriptions.push(
        vscode.window.registerWebviewViewProvider(IAAgentViewProvider.viewType, new IAAgentViewProvider(extensionUri))
    );
}

export function deactivate() {}

class IAAgentViewProvider implements vscode.WebviewViewProvider {
    public static readonly viewType = 'iaAgentView';
    constructor(private readonly extensionUri: vscode.Uri) {}

    public resolveWebviewView(webviewView: vscode.WebviewView): void {
        webviewView.webview.options = {
            enableScripts: true,
            localResourceRoots: [vscode.Uri.joinPath(this.extensionUri, 'resources')]
        };
        webviewView.webview.html = getHtmlContentFromFile(this.extensionUri, 'chat.html', webviewView.webview);
    }
}

function getHtmlContentFromFile(extensionUri: vscode.Uri, fileName: string, webview: vscode.Webview): string {
    const filePath = vscode.Uri.joinPath(extensionUri, 'resources', 'views', fileName);

    //  Init webview configuration
    let html = fs.readFileSync(filePath.fsPath, 'utf8');
    html = html.replace(/\${webview\.cspSource}/g, webview.cspSource);

    // Icons paths
    const iconUri = webview.asWebviewUri(vscode.Uri.joinPath(extensionUri, 'resources', 'images', 'icons', 'ia-agent-icon.svg'));
    html = html.replace(/\${ia_agent_icon.svg}/g, iconUri.toString());

    const icon2 = webview.asWebviewUri(vscode.Uri.joinPath(extensionUri, 'resources', 'images', 'icons', 'ia-agent-icon-2.svg'));
    html = html.replace(/\${ia_agent_icon-2.svg}/g, icon2.toString());

    
    const logo = webview.asWebviewUri(vscode.Uri.joinPath(extensionUri, 'resources', 'images', 'icons', 'ia-agent-logo.svg'));
    html = html.replace(/\${ia_agent_logo.svg}/g, logo.toString());

    const resources_js = webview.asWebviewUri(vscode.Uri.joinPath(extensionUri, 'resources', 'js', 'app.js'));
    return html.replace(/\${resource_js}/g, resources_js.toString());
}
