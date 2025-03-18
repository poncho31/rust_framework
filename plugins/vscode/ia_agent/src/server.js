// Charge les variables d'environnement depuis .env (si présent)
require('dotenv').config();

const express = require('express');
const bodyParser = require('body-parser');
const cors = require('cors');
const axios = require('axios');

const app = express();
const port = 8000;

app.use(cors());
app.use(bodyParser.json());

// Choix du provider (défaut : openai)
const LLM_PROVIDER = process.env.LLM_PROVIDER || "openai";

// Vérification de la clé API en fonction du provider choisi
if (LLM_PROVIDER === "openai") {
  if (!process.env.OPENAI_API_KEY || process.env.OPENAI_API_KEY.trim() === "") {
    console.error("Erreur: OpenAI API key est vide. Veuillez définir process.env.OPENAI_API_KEY.");
  }
} else if (LLM_PROVIDER === "mistral") {
  if (!process.env.MISTRAL_API_KEY || process.env.MISTRAL_API_KEY.trim() === "") {
    console.error("Erreur: Mistral API key est vide. Veuillez définir process.env.MISTRAL_API_KEY.");
  }
} else {
  console.error(`Erreur: Provider inconnu '${LLM_PROVIDER}'.`);
}

app.post('/chat', async (req, res) => {
  const prompt = req.body.text || '';
  if (!prompt.trim()) {
    return res.status(400).json({ response: 'Erreur: le prompt est vide.' });
  }
  try {
    let responseText;
    if (LLM_PROVIDER === "openai") {
      const response = await axios.post(
        'https://api.openai.com/v1/chat/completions',
        {
          model: "gpt-3.5-turbo",
          messages: [{ role: "user", content: prompt }],
          temperature: 0.2,
        },
        {
          headers: {
            "Authorization": `Bearer ${process.env.OPENAI_API_KEY}`,
            "Content-Type": "application/json"
          }
        }
      );
      responseText = response.data.choices[0].message.content;
    } else if (LLM_PROVIDER === "mistral") {
      // Exemple : adaptation pour Mistral (à ajuster selon l'API réelle de Mistral)
      const response = await axios.post(
        'https://api.mistral.ai/v1/chat/completions',
        {
          model: "mistral-7b", // Exemple de modèle pour Mistral
          messages: [{ role: "user", content: prompt }],
          temperature: 0.2,
        },
        {
          headers: {
            "Authorization": `Bearer ${process.env.MISTRAL_API_KEY}`,
            "Content-Type": "application/json"
          }
        }
      );
      responseText = response.data.choices[0].message.content + LLM_PROVIDER
    } else {
      return res.status(500).json({ response: `Erreur: Provider ${LLM_PROVIDER} non supporté.` });
    }
    res.json({ response: responseText });
  } catch (error) {
    console.error("Erreur lors de la requête axios:", error.response ? error.response.data : error.message);
    let errorMessage = error.message;
    if (error.response && error.response.data) {
      errorMessage += ' - ' + JSON.stringify(error.response.data);
    }
    res.status(500).json({ response: 'Erreur lors de la requête vers le provider: ' + errorMessage });
  }
});

app.listen(port, () => {
  console.log(`Serveur IA lancé sur http://localhost:${port} en utilisant le provider ${LLM_PROVIDER}`);
});
