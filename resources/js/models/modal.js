class Modal {
    constructor(openButtonId, modalId, closeButtonId, closeFooterButtonId, backgroundClass, listContainerSelector) {
        this.openButton        = document.getElementById(openButtonId);
        this.modal             = document.getElementById(modalId);
        this.closeButton       = document.getElementById(closeButtonId);
        this.closeFooterButton = document.getElementById(closeFooterButtonId);
        this.modalBackground   = document.querySelector(backgroundClass);
        this.listContainer     = document.querySelector(listContainerSelector);

        this.initEventListeners();
        this.updateButtonPosition(); // Initialiser la position au chargement

        // Écouter les événements de défilement et de redimensionnement
        window.addEventListener('scroll', () => this.updateButtonPosition());
        window.addEventListener('resize', () => this.updateButtonPosition());
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
        if (this.openButton) {
            this.openButton.addEventListener('click', () => this.openModal());
        }

        if (this.closeButton) {
            this.closeButton.addEventListener('click', () => this.closeModal());
        }

        if (this.closeFooterButton) {
            this.closeFooterButton.addEventListener('click', () => this.closeModal());
        }

        if (this.modalBackground) {
            this.modalBackground.addEventListener('click', () => this.closeModal());
        }
    }

    // Met à jour la position du bouton pour qu'il suive le comportement attendu
    updateButtonPosition() {
        const containerRect = this.listContainer.getBoundingClientRect();
        const viewportHeight = window.innerHeight;

        this.openButton.style.transition = "all 0.3s ease-in-out";
        // Le haut de la liste est visible dans la fenêtre d'affichage
        if (containerRect.top > 0 && containerRect.top > viewportHeight - 70) {
            this.openButton.style.position = 'fixed';
            this.openButton.style.bottom   = 'auto';
        }
        // Le début de la liste est atteint => le bouton de modal apparait
        else if (containerRect.bottom > viewportHeight) {
            this.openButton.style.position = 'fixed';
            this.openButton.style.bottom   = '20px';
            this.openButton.style.right    = '20px';
            this.openButton.style.top      = 'auto';
        }
        // La fin de la liste est atteinte
        else {
            this.openButton.style.position = 'relative';
            this.openButton.style.bottom   = '0px';
            this.openButton.style.right    = '0px';
            this.openButton.style.top      = 'auto';
        }
    }
}

// Exporter la classe Modal
export { Modal };
