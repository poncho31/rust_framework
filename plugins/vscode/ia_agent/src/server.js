const express = require('express');
const bodyParser = require('body-parser');
const cors = require('cors');
const { Configuration, OpenAIApi } = require('openai');

const app = express();
const port = 8000;

app.use(cors());
app.use(bodyParser.json());

// Vérification de la clé API OpenAI
if (!process.env.OPENAI_API_KEY || process.env.OPENAI_API_KEY.trim() === "") {
  console.error("Erreur: OpenAI API key est vide. Veuillez définir process.env.OPENAI_API_KEY.");
}

const configuration = new Configuration({
  apiKey: process.env.OPENAI_API_KEY,
});
const openai = new OpenAIApi(configuration);

app.post('/chat', async (req, res) => {
  const prompt = req.body.text || '';
  if (!prompt.trim()) {
    return res.status(400).json({ response: 'Erreur: le prompt est vide.' });
  }
  try {
    const completion = await openai.createChatCompletion({
      model: "gpt-3.5-turbo",
      messages: [{ role: "user", content: prompt }],
      temperature: 0.2,
    });
    const responseText = completion.data.choices[0].message.content;
    res.json({ response: responseText });
  } catch (error) {
    console.error("Erreur lors de createChatCompletion:", error);
    let errorMessage = error.message;
    if (error.response && error.response.data) {
      errorMessage += ' - ' + JSON.stringify(error.response.data);
    }
    res.status(500).json({ response: 'Erreur lors de la requête OpenAI: ' + errorMessage });
  }
});

app.listen(port, () => {
  console.log(`Serveur IA lancé sur http://localhost:${port}`);
});
