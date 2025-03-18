from fastapi import FastAPI, HTTPException
from pydantic import BaseModel
from transformers import AutoTokenizer, AutoModelForCausalLM
import torch

# Créer une instance de FastAPI
app = FastAPI()

# Définir le schéma de la requête
class PromptRequest(BaseModel):
    prompt: str

# Charger le tokenizer et le modèle Mistral
# Remplacez "mistralai/Mistral-7B-v0.1" par le nom du modèle que vous souhaitez utiliser, s'il est disponible.
model_name = "mistralai/Mistral-7B-v0.1"
tokenizer = AutoTokenizer.from_pretrained(model_name)
model = AutoModelForCausalLM.from_pretrained(
    model_name,
    torch_dtype=torch.float16,   # ou torch.float32 si nécessaire
    device_map="auto"            # Permet de charger le modèle sur GPU si disponible
)

@app.post("/chat")
async def chat_endpoint(request: PromptRequest):
    prompt_text = request.prompt
    if not prompt_text.strip():
        raise HTTPException(status_code=400, detail="Le prompt est vide.")
    
    # Tokenisation du prompt
    inputs = tokenizer(prompt_text, return_tensors="pt").to(model.device)
    
    # Génération de la réponse (vous pouvez ajuster max_new_tokens et d'autres paramètres)
    outputs = model.generate(**inputs, max_new_tokens=100)
    
    # Décodage de la réponse
    response_text = tokenizer.decode(outputs[0], skip_special_tokens=True)
    return {"response": response_text}

if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host="127.0.0.1", port=8001)
