"use strict";
// Refactored version that clearly divides responsibilities:
// - Starting the IA server
// - Creating the chat webview panel
// - Registering commands (for opening chat, sidebar, and new windows)
// - Providing the sidebar view and utility to load HTML content
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
/*===============================
   IA Server Initialization
===============================*/
// Spawns the local IA server process.
function startServer(context) {
    const serverPath = path.join(context.extensionPath, 'src', 'server.js');
    const serverProcess = cp.spawn('node', [serverPath], { stdio: 'inherit' });
    context.subscriptions.push({ dispose: () => serverProcess.kill() });
}
/*===============================
   Chat Webview Panel Creation
===============================*/
// Creates the chat webview panel.
function createChatWebview(extensionUri) {
    const panel = vscode.window.createWebviewPanel('iaAgentChat', 'Chat with IA Agent', vscode.ViewColumn.One, {
        enableScripts: true,
        localResourceRoots: [vscode.Uri.joinPath(extensionUri, 'resources')]
    });
    panel.webview.html = getHtmlContentFromFile(extensionUri, 'chat.html', panel.webview);
    // Listen for messages from the webview
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
        else if (message.command === 'runCommand') {
            // Adapted: Execute the requested command.
            vscode.commands.executeCommand(message.commandName);
            return;
        }
    });
    return panel;
}
/*===============================
   Command Registration Functions
===============================*/
// Updated registerOpenChatCommand to simply open the chat webview panel without spawning a new VS Code process.
function registerOpenChatCommand(context, extensionUri) {
    context.subscriptions.push(vscode.commands.registerCommand('ia-agent.openChat', () => {
        // Directly create the chat webview panel in the current window.
        createChatWebview(extensionUri);
    }));
}
// Registers the openSidebar command.
function registerOpenSidebarCommand() {
    vscode.commands.registerCommand('ia-agent.openSidebar', () => vscode.commands.executeCommand('workbench.view.iaAgent'));
}
// Registers commands that open a new window via various strategies.
function registerNewWindowCommands(context) {
    // Option 1: Use process.spawn with CHAT_NEW_WINDOW flag.
    context.subscriptions.push(vscode.commands.registerCommand('ia-agent.openChatNewWindow', () => {
        const execPath = process.execPath;
        cp.spawn(execPath, ['--new-window', '--command', 'ia-agent.openChat'], {
            detached: true,
            env: { ...process.env, CHAT_NEW_WINDOW: '1', CHAT_AUTOOPEN: '1' }
        }).unref();
    }));
    // Option 2: Use VS Code CLI ("code") and a simple spawn.
    context.subscriptions.push(vscode.commands.registerCommand('ia-agent.openChatNewWindow2', () => {
        const codeCli = 'code'; // must be in PATH
        const cmd = `${codeCli} --new-window --command "ia-agent.openChat"`;
        cp.exec(cmd, (err) => {
            if (err) {
                vscode.window.showErrorMessage(`Error opening new window: ${err.message}`);
            }
        });
    }));
    // Option 3: Use VS Code CLI with extension development path.
    context.subscriptions.push(vscode.commands.registerCommand('ia-agent.openChatNewWindowCLI', () => {
        const extPath = context.extensionPath;
        const cmd = `code --new-window --extensionDevelopmentPath="${extPath}" --command "ia-agent.openChat"`;
        cp.exec(cmd, (err) => {
            if (err) {
                vscode.window.showErrorMessage(`Error opening new window: ${err.message}`);
            }
        });
    }));
}
// Registers the webview view provider for the IA Agent sidebar.
function registerWebviewView(context, extensionUri) {
    context.subscriptions.push(vscode.window.registerWebviewViewProvider(IAAgentViewProvider.viewType, new IAAgentViewProvider(extensionUri)));
}
/*===============================
   Extension Activation
===============================*/
function activate(context) {
    const extensionUri = context.extensionUri;
    // Start the local IA server.
    startServer(context);
    // Register core commands.
    registerOpenChatCommand(context, extensionUri);
    registerOpenSidebarCommand();
    registerNewWindowCommands(context);
    // Register the sidebar view.
    registerWebviewView(context, extensionUri);
    // If in a new window with the auto-open flag, immediately open chat
    if (process.env.CHAT_NEW_WINDOW && process.env.CHAT_AUTOOPEN === '1') {
        createChatWebview(extensionUri);
    }
}
function deactivate() { }
/*===============================
   Webview View Provider Class
===============================*/
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
/*===============================
   Utility Functions
===============================*/
// Reads the chat HTML file and replaces placeholders (e.g. URIs).
function getHtmlContentFromFile(extensionUri, fileName, webview) {
    const filePath = vscode.Uri.joinPath(extensionUri, 'resources', 'views', fileName);
    let html = fs.readFileSync(filePath.fsPath, 'utf8');
    // Substitute webview CSP source.
    html = html.replace(/\${webview\.cspSource}/g, webview.cspSource);
    // Resolve icon and script URIs.
    const iconUri = webview.asWebviewUri(vscode.Uri.joinPath(extensionUri, 'resources', 'images', 'icons', 'ia-agent-icon.svg'));
    html = html.replace(/\${ia_agent_icon\.svg}/g, iconUri.toString());
    const icon2Uri = webview.asWebviewUri(vscode.Uri.joinPath(extensionUri, 'resources', 'images', 'icons', 'ia-agent-icon-2.svg'));
    html = html.replace(/\${ia_agent_icon-2\.svg}/g, icon2Uri.toString());
    const logoUri = webview.asWebviewUri(vscode.Uri.joinPath(extensionUri, 'resources', 'images', 'icons', 'ia-agent-logo.svg'));
    html = html.replace(/\${ia_agent_logo\.svg}/g, logoUri.toString());
    const resourcesJsUri = webview.asWebviewUri(vscode.Uri.joinPath(extensionUri, 'resources', 'js', 'app.js'));
    return html.replace(/\${resource_js}/g, resourcesJsUri.toString());
}
//# sourceMappingURL=extension.js.map