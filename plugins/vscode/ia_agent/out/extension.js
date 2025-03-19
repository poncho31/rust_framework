"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || (function () {
    var ownKeys = function(o) {
        ownKeys = Object.getOwnPropertyNames || function (o) {
            var ar = [];
            for (var k in o) if (Object.prototype.hasOwnProperty.call(o, k)) ar[ar.length] = k;
            return ar;
        };
        return ownKeys(o);
    };
    return function (mod) {
        if (mod && mod.__esModule) return mod;
        var result = {};
        if (mod != null) for (var k = ownKeys(mod), i = 0; i < k.length; i++) if (k[i] !== "default") __createBinding(result, mod, k[i]);
        __setModuleDefault(result, mod);
        return result;
    };
})();
Object.defineProperty(exports, "__esModule", { value: true });
exports.activate = activate;
exports.deactivate = deactivate;
const vscode = __importStar(require("vscode"));
const fs = __importStar(require("fs"));
const cp = __importStar(require("child_process"));
const path = __importStar(require("path"));
function activate(context) {
    const extensionUri = context.extensionUri;
    // Lancer le serveur IA local
    const serverProcess = cp.spawn('node', [path.join(context.extensionPath, 'src', 'server.js')], { stdio: 'inherit' });
    context.subscriptions.push({ dispose: () => serverProcess.kill() });
    // Enregistrer les commandes
    context.subscriptions.push(vscode.commands.registerCommand('ia-agent.helloWorld', () => vscode.window.showInformationMessage('Hello World from ia_agent!')), vscode.commands.registerCommand('ia-agent.openChat', () => {
        const panel = vscode.window.createWebviewPanel('iaAgentChat', 'Chat with IA Agent', vscode.ViewColumn.One, {
            enableScripts: true,
            localResourceRoots: [vscode.Uri.joinPath(extensionUri, 'resources')]
        });
        panel.webview.html = getHtmlContentFromFile(extensionUri, 'chat.html', panel.webview);
        panel.webview.onDidReceiveMessage(async (message) => {
            if (message.command === 'send') {
                try {
                    const response = await fetch('http://localhost:8000/chat', {
                        method: 'POST',
                        headers: { 'Content-Type': 'application/json' },
                        body: JSON.stringify({ text: message.text })
                    });
                    const data = await response.json();
                    panel.webview.postMessage({ command: 'response', text: data.response });
                }
                catch (error) {
                    panel.webview.postMessage({ command: 'response', text: 'Error: ' + error.message });
                }
            }
        });
    }), vscode.commands.registerCommand('ia-agent.openSidebar', () => vscode.commands.executeCommand('workbench.view.iaAgent')));
    // Enregistrer le WebviewViewProvider
    context.subscriptions.push(vscode.window.registerWebviewViewProvider(IAAgentViewProvider.viewType, new IAAgentViewProvider(extensionUri)));
}
function deactivate() { }
class IAAgentViewProvider {
    extensionUri;
    static viewType = 'iaAgentView';
    constructor(extensionUri) {
        this.extensionUri = extensionUri;
    }
    resolveWebviewView(webviewView) {
        webviewView.webview.options = {
            enableScripts: true,
            localResourceRoots: [vscode.Uri.joinPath(this.extensionUri, 'resources')]
        };
        webviewView.webview.html = getHtmlContentFromFile(this.extensionUri, 'chat.html', webviewView.webview);
    }
}
function getHtmlContentFromFile(extensionUri, fileName, webview) {
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
//# sourceMappingURL=extension.js.map