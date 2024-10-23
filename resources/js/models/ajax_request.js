export class AjaxRequest {
    constructor() {
        this.init();
    }

    // Initialisation : Ajouter un écouteur d'événements sur tous les formulaires de la page
    init() {
        document.addEventListener('submit', (event) => {
            event.preventDefault(); // Empêcher le comportement par défaut (soumission du form)
            const form = event.target;

            // Récupérer les données du formulaire
            const formData   = new FormData(form);
            const form_data_json= form.getAttribute('data-json');
            const action        = form.getAttribute('action');
            const method        = form.getAttribute('method') || 'POST';

            // Appeler la méthode pour envoyer la requête
            let response= this.sendRequest(action, method, formData)
                .then((response) => this.handleResponse(response, form_data_json))
                .catch((error) => this.handleError(error, form));
        });
    }

    // Envoie la requête AJAX
    async sendRequest(url, method, formData) {
        const formBody = new URLSearchParams();
        formData.forEach((value, key) => {
            formBody.append(key, value);
        });

        const options = {
            method: method.toUpperCase(),
            headers: {
                'Content-Type': 'application/x-www-form-urlencoded', // Envoyer les données sous ce format
            },
            body: formBody.toString()  // Convertir les données en chaîne URL encodée
        };

        const response = await fetch(url, options);

        console.log("AJAX REQUEST RESPONSE",response);

        if (!response.ok) {
            throw new Error(`HTTP error! Status: ${response.status}`);
        }

        return response.json();
    }


    // Gérer la réponse de la requête
    handleResponse(response, form_data_json) {
        console.log('Requête réussie :', response);

        let   data_json = JSON.parse(form_data_json);
        if(data_json.id_data_add !== undefined){
            let tag_id = document.getElementById(data_json.id_data_add);
            tag_id.insertAdjacentHTML('afterbegin', response.data);
        }
    }

    // Gérer les erreurs de la requête
    handleError(error, form) {
        console.error('Erreur lors de la requête :', error);
        // Ici, tu peux ajouter ton propre comportement en cas d'erreur (afficher un message d'erreur, etc.)
    }
}
