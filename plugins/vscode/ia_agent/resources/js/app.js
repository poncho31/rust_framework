import { initChat } from "./chat.js";

console.log("app.js loaded"); // Debug

// Rendre initChat accessible globalement
window.initChat = initChat;

// ...existing code ou autres appels...
const vscode = acquireVsCodeApi();
