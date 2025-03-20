import * as dotenv from 'dotenv';
import * as path from 'path';
dotenv.config({ path: path.join(__dirname, '..', '.env') });
// Refactored version that clearly divides responsibilities:
// - Starting the IA server
// - Creating the chat webview panel
// - Registering commands (for opening chat, sidebar, and new windows)
// - Providing the sidebar view and utility to load HTML content

import * as vscode from 'vscode';
import * as fs from 'fs';
import * as cp from 'child_process';

/*===============================
   IA Server Initialization
===============================*/
// Spawns the local IA server process.
function startServer(context: vscode.ExtensionContext) {
    const provider = (process.env.LLM_PROVIDER || "openai").trim();
    console.log(`Starting IA server with provider: ${provider}`);
    if (provider === "mistral") {
        //  py -3.10 -m venv venv
        // .\venv\Scripts\Activate.ps1
        // .\venv\Scripts\python.exe -m pip install -r requirements.txt
        // .\venv\Scripts\python.exe server_mistral.py

        const mistralDir = path.join(context.extensionPath, 'plugins', 'mistral');
        const serverPath = path.join(mistralDir, 'server_mistral.py');
        const venvPython = path.join(mistralDir, 'venv', 'Scripts', 'python.exe');
        
        // Exécute la commande simple pour créer le virtualenv et installer les dépendances
        cp.execSync('py -m venv venv && venv\\Scripts\\python.exe -m pip install -r requirements.txt', { cwd: mistralDir, stdio: 'inherit' });
        
        const pythonProcess = cp.spawn(venvPython, [serverPath], { cwd: mistralDir, stdio: 'inherit' });
        context.subscriptions.push({ dispose: () => pythonProcess.kill() });
    } else {
        const serverPath = path.join(context.extensionPath, 'src', 'server.js');
        const serverProcess = cp.spawn('node', [serverPath], { stdio: 'inherit' });
        context.subscriptions.push({ dispose: () => serverProcess.kill() });
    }
}

/*===============================
   Chat Webview Panel Creation
===============================*/
// Creates the chat webview panel.
function createChatWebview(extensionUri: vscode.Uri): vscode.WebviewPanel {
    const panel = vscode.window.createWebviewPanel(
        'iaAgentChat',
        'Chat with IA Agent',
        vscode.ViewColumn.One,
        {
            enableScripts: true,
            localResourceRoots: [vscode.Uri.joinPath(extensionUri, 'resources')]
        }
    );
    panel.webview.html = getHtmlContentFromFile(extensionUri, 'chat.html', panel.webview);

    // Listen for messages from the webview
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
        } else if (message.command === 'runCommand') {
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
function registerOpenChatCommand(context: vscode.ExtensionContext, extensionUri: vscode.Uri) {
    context.subscriptions.push(
        vscode.commands.registerCommand('ia-agent.openChat', () => {
            // Directly create the chat webview panel in the current window.
            createChatWebview(extensionUri);
        })
    );
}

// Registers the openSidebar command.
function registerOpenSidebarCommand() {
    vscode.commands.registerCommand('ia-agent.openSidebar', () =>
        vscode.commands.executeCommand('workbench.view.iaAgent')
    );
}

// Registers commands that open a new window via various strategies.
function registerNewWindowCommands(context: vscode.ExtensionContext) {
    // Option 1: Use process.spawn with CHAT_NEW_WINDOW flag.
    context.subscriptions.push(
        vscode.commands.registerCommand('ia-agent.openChatNewWindow', () => {
            const execPath = process.execPath;
            cp.spawn(execPath, ['--new-window', '--command', 'ia-agent.openChat'], {
                detached: true,
                env: { ...process.env, CHAT_NEW_WINDOW: '1', CHAT_AUTOOPEN: '1' }
            }).unref();
        })
    );
    // Option 2: Use VS Code CLI ("code") and a simple spawn.
    context.subscriptions.push(
        vscode.commands.registerCommand('ia-agent.openChatNewWindow2', () => {
            const codeCli = 'code'; // must be in PATH
            const cmd = `${codeCli} --new-window --command "ia-agent.openChat"`;
            cp.exec(cmd, (err) => {
                if (err) {
                    vscode.window.showErrorMessage(`Error opening new window: ${err.message}`);
                }
            });
        })
    );
    // Option 3: Use VS Code CLI with extension development path.
    context.subscriptions.push(
        vscode.commands.registerCommand('ia-agent.openChatNewWindowCLI', () => {
            const extPath = context.extensionPath;
            const cmd = `code --new-window --extensionDevelopmentPath="${extPath}" --command "ia-agent.openChat"`;
            cp.exec(cmd, (err) => {
                if (err) {
                    vscode.window.showErrorMessage(`Error opening new window: ${err.message}`);
                }
            });
        })
    );
}

// Registers the webview view provider for the IA Agent sidebar.
function registerWebviewView(context: vscode.ExtensionContext, extensionUri: vscode.Uri) {
    context.subscriptions.push(
        vscode.window.registerWebviewViewProvider(
            IAAgentViewProvider.viewType,
            new IAAgentViewProvider(extensionUri)
        )
    );
}

/*===============================
   Extension Activation
===============================*/
export function activate(context: vscode.ExtensionContext) {
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

export function deactivate() {}

/*===============================
   Webview View Provider Class
===============================*/
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

/*===============================
   Utility Functions
===============================*/
// Reads the chat HTML file and replaces placeholders (e.g. URIs).
function getHtmlContentFromFile(extensionUri: vscode.Uri, fileName: string, webview: vscode.Webview): string {
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
