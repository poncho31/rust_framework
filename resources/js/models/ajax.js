// Classe JavaScript pour gérer les requêtes AJAX
export class Ajax {
    constructor(ajax_data) {
        let data_parse    = JSON.parse(ajax_data);
        this.ajax         = data_parse.ajax_options;
        this.form         = data_parse.form;
        console.log(data_parse);
    }
    async run_ajax() {
        try {
            // Préparation de la requête AJAX
            const response = await fetch(this.form.action, {
                method: this.form.method,
                headers: {
                    'Content-Type': this.ajax.request_options.data_type,
                    ...this.ajax.custom_headers,
                },
                body  : JSON.stringify(this.form.fields),
                cache : this.ajax.request_options.cache ? 'default' : 'no-store',
            });

            // Gestion de la réponse
            if (!response.ok) {
                throw new Error(`HTTP error! status: ${response.status}`);
            }

            const data = await response.json();

            // Appel du callback de succès
            onFormSuccess(this.ajax.callbacks.success, data);

            console.error("AJAX SUCCESS", data);
        }
        catch (error) {
            console.error("AJAX ERROR", error);
            
        } finally {
            console.error("AJAX FINALLY");
        }
    }
}

// Callback avant soumission
function onFormBeforeSubmit() {
    console.log('Form submission is about to start.');
}

// Callback en cas de succès
function onFormSuccess(success_callback,data) {
    console.log('Success callback           :', success_callback);
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
