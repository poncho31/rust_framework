require('dotenv').config({ path: __dirname + '/../.env' }); // Assurez-vous que ce chemin est correct

console.log("Chargement des variables d'environnement...");
console.log("Chemin du fichier .env :", __dirname + '/../.env'); // Debug
console.log("OPENAI_API_KEY:", process.env.OPENAI_API_KEY); // Debug

if (!process.env.OPENAI_API_KEY || process.env.OPENAI_API_KEY.trim() === "") {
  console.error("Erreur: OpenAI API key est manquante ou vide.");
  process.exit(1); // Arrête le serveur si la clé est manquante
}

const express = require('express');
const bodyParser = require('body-parser');
const cors = require('cors');
const routes = require('./routes');

const app = express();
const port = 8000;

app.use(cors());
app.use(bodyParser.json());

app.use('/', routes);

app.listen(port, () => {
  console.log(`Serveur IA lancé sur http://localhost:${port} en utilisant le provider ${process.env.LLM_PROVIDER}`);
});