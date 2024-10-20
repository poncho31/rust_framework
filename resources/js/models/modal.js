class Modal {
    constructor(openButtonId, modalId, closeButtonId, closeFooterButtonId, backgroundClass) {
        this.openButton = document.getElementById(openButtonId);
        this.modal = document.getElementById(modalId);
        this.closeButton = document.getElementById(closeButtonId);
        this.closeFooterButton = document.getElementById(closeFooterButtonId);
        this.modalBackground = document.querySelector(backgroundClass);

        this.initEventListeners();
    }

    // Méthode pour ouvrir la modale
    openModal() {
        this.modal.classList.add('is-active');
    }

    // Méthode pour fermer la modale
    closeModal() {
        this.modal.classList.remove('is-active');
    }

    // Ajoute tous les écouteurs d'événements
    initEventListeners() {
        this.openButton.addEventListener('click', () => this.openModal());

        this.closeButton.addEventListener('click', () => this.closeModal());

        this.closeFooterButton.addEventListener('click', () => this.closeModal());

        this.modalBackground.addEventListener('click', () => this.closeModal());
    }
}

// Exporter la classe Modal
export { Modal };
