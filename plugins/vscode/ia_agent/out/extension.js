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
        const panel = vscode.window.createWebviewPanel('iaAgentChat', 'Chat with IA Agent', vscode.ViewColumn.One, { enableScripts: true });
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
                    const data = await response.json();
                    panel.webview.postMessage({ command: 'response', text: data.response });
                }
                catch (error) {
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
    context.subscriptions.push(vscode.window.registerWebviewViewProvider(IAAgentViewProvider.viewType, viewProvider));
    console.log('IAAgentViewProvider enregistré');
}
function deactivate() { }
class IAAgentViewProvider {
    extensionUri;
    static viewType = 'iaAgentView';
    _view;
    constructor(extensionUri) {
        this.extensionUri = extensionUri;
    }
    resolveWebviewView(webviewView, _context, _token) {
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
function getHtmlContentFromFile(extensionUri, fileName, webview) {
    const filePath = vscode.Uri.joinPath(extensionUri, 'resources', 'views', fileName);
    let html = fs.readFileSync(filePath.fsPath, 'utf8');
    // Remplacer ${webview.cspSource} par la valeur correcte dans le HTML
    html = html.replace(/\${webview\.cspSource}/g, webview.cspSource);
    return html;
}
//# sourceMappingURL=extension.js.map