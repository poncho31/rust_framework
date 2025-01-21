// Classe JavaScript pour gérer les requêtes AJAX
export class Ajax {
    constructor(ajaxOptions) {
        this.ajaxOptions = ajaxOptions;
    }

    async run_ajax() {
        try {
            // Préparation de la requête AJAX
            const response = await fetch(this.ajaxOptions.form.action, {
                method: this.ajaxOptions.form.method,
                headers: {
                    'Content-Type': this.ajaxOptions.request_options.data_type,
                    ...this.ajaxOptions.custom_headers,
                },
                body: JSON.stringify(this.ajaxOptions.form.fields),
                cache: this.ajaxOptions.request_options.cache ? 'default' : 'no-store',
            });

            // Gestion de la réponse
            if (!response.ok) {
                throw new Error(`HTTP error! status: ${response.status}`);
            }

            const data = await response.json();

            // Appel du callback de succès
            if (typeof window[this.ajaxOptions.callbacks.success] === 'function') {
                window[this.ajaxOptions.callbacks.success](data);
            }
        } catch (error) {
            
        } finally {

        }
    }
}

// Callback avant soumission
function onFormBeforeSubmit() {
    console.log('Form submission is about to start.');
}

// Callback en cas de succès
function onFormSuccess(data) {
    console.log('Form submitted successfully:', data);
}

// Callback en cas d'erreur
function onFormError(error) {
    console.error('Error occurred during form submission:', error);
}

// Callback complet
function onFormComplete() {
    console.log('Form submission process completed.');
}
