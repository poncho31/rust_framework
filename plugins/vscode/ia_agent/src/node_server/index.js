require('dotenv').config({ path: __dirname + '/../.env' });
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