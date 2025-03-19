export function initChat(btnId) {
    console.log("initChat called with id:", btnId); // Debug
    const chatLog = document.getElementById('chat-log');
    const chatInput = document.getElementById('chat-input');
    const sendBtn = document.getElementById(btnId);

    if (!chatLog || !chatInput || !sendBtn) {
        console.error("Missing one or more required elements: chatLog, chatInput, sendBtn");
        return;
    }

    function appendMessage(sender, text) {
        const el = document.createElement('div');
        el.innerHTML = `<strong>${sender}:</strong> ${text}`;
        chatLog.appendChild(el);
        chatLog.scrollTop = chatLog.scrollHeight;
    }

    function sendMessage() {
        console.log("sendMessage triggered with input:", chatInput.value); // Debug
        const text = chatInput.value.trim();
        if (!text) return;
        appendMessage('Vous', text);
        fetch('http://127.0.0.1:8000/chat', {
            method: 'POST',
            mode: 'cors',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ text })
        })
        .then(res => {
            if (!res.ok) throw new Error(`Erreur HTTP: ${res.status}`);
            return res.json();
        })
        .then(data => appendMessage('Assistant', data.response))
        .catch(err => appendMessage('Assistant', 'Erreur: ' + err.message));
        chatInput.value = "";
    }

    // Appel immédiat lors du clic sur le bouton identifié
    sendMessage();
}