const path = require('path');
require('dotenv').config({ path: path.join(__dirname, '..', '..', '.env') });

const express = require('express');
const bodyParser = require('body-parser');
const readline = require('readline');

let pipeline;
try {
    pipeline = require('@xenova/transformers').pipeline;
} catch (err) {
    console.error("Module '@xenova/transformers' n'est pas installé. Veuillez exécuter 'npm install @xenova/transformers' dans le répertoire approprié.");
    process.exit(1);
}

const app = express();
const port = process.env.MISTRAL_PORT || 8011;
const defaultText = process.argv[2] || '';

// Déclaration globale du générateur IA
let generator = null;

// Fonction asynchrone de chargement du modèle depuis HuggingFace
async function loadModel() {
    let modelName = process.env.HUGGINGFACE_MODEL || "mistralai/Mistral-Small-3.1-24B-Instruct-2503";
    // Remplacer distilgpt2 ou gpt2 par xenova/gpt2 qui est disponible en ONNX
    if (modelName.toLowerCase().includes('distilgpt2') || modelName.toLowerCase() === 'gpt2') {
        console.warn("Le modèle demandé n'est pas disponible en ONNX. Utilisation de xenova/gpt2 à la place.");
        modelName = "xenova/gpt2";
    }
    const authToken = process.env.MISTRAL_API_TOKEN_NAME;
    console.log(`Chargement du modèle ${modelName} depuis HuggingFace...`);
    try {
        generator = await pipeline('text-generation', modelName, { useAuthToken: authToken });
        console.log("Modèle chargé !");
    } catch (err) {
        console.error("Erreur lors du chargement du modèle :", err);
        console.error("Assurez-vous que le modèle est disponible au format ONNX pour @xenova/transformers.");
        // ...vous pouvez choisir d'arrêter l'application ou de réessayer ultérieurement...
    }
}
loadModel().catch(err => {
    console.error("Erreur lors du chargement du modèle :", err);
});

app.use(bodyParser.json());

app.post('/chat', async (req, res) => {
    // Utilise le texte depuis req.body ou le fallback defaultText
    const prompt = req.body.messages ? req.body.messages[0].content : (req.body.text || defaultText);
    if (!prompt.trim()) {
        return res.status(400).json({ error: "Le prompt est vide." });
    }
    if (!generator) {
        return res.status(500).json({ error: "Modèle non chargé, réessayez plus tard." });
    }
    try {
        // Utiliser le générateur pour produire la réponse
        let output = await generator(prompt, { max_new_tokens: 100 });
        const responseText = output[0].generated_text;
        res.json({
            choices: [
                { message: { content: responseText } }
            ]
        });
    } catch (err) {
        console.error("Erreur d'inférence :", err);
        res.status(500).json({ error: "Erreur lors de l'inférence du modèle." });
    }
});

app.listen(port, () => {
    console.log(`Mistral server (Node) lancé sur http://localhost:${port}`);
    console.log("Tapez votre prompt dans le terminal:");
    
    // Interface de dialogue dans le terminal
    const rl = readline.createInterface({
        input: process.stdin,
        output: process.stdout,
        prompt: "Mistral> "
    });
    rl.prompt();
    rl.on('line', async (line) => {
        const input = line.trim();
        if (!input) {
            console.log("Le prompt est vide.");
        } else {
            if (!generator) {
                console.log("Modèle non chargé, réessayez plus tard.");
            } else {
                try {
                    let output = await generator(input, { max_new_tokens: 100 });
                    console.log(output[0].generated_text);
                } catch (err) {
                    console.error("Erreur d'inférence :", err);
                }
            }
        }
        rl.prompt();
    });
});
