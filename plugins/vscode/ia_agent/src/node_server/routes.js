const express = require('express');
const router = express.Router();
const { callLLM } = require('./llmClient.js');

router.post('/chat', async (req, res) => {
  const prompt = req.body.text || '';
  if (!prompt.trim()) {
    return res.status(400).json({ response: 'Erreur: le prompt est vide.' });
  }
  try {
    const responseText = await callLLM(prompt);
    res.json({ response: responseText });
  } catch (error) {
    console.error("Erreur :", error.message);
    res.status(500).json({ response: 'Erreur lors de la requête vers le provider: ' + error.message });
  }
});

module.exports = router;
