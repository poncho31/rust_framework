const axios = require('axios');
const LLM_PROVIDER = process.env.LLM_PROVIDER;

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

async function callLLM(prompt) {
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
    const response = await axios.post(
      'https://api.mistral.ai/v1/chat/completions',
      {
        model: "mistral-7b",
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
    responseText = response.data.choices[0].message.content + LLM_PROVIDER;
  } else {
    throw new Error(`Provider ${LLM_PROVIDER} non supporté.`);
  }
  return responseText;
}

module.exports = { callLLM };
