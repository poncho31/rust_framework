import { Modal }       from './models/modal.js';
import { AjaxRequest } from './models/ajax_request.js';

window.Modal       = Modal;
window.AjaxRequest = AjaxRequest;

document.addEventListener('DOMContentLoaded', () => {
    // Sélectionner le bouton "burger" et le menu de navigation
    const burger     = document.querySelector('.navbar-burger');
    const navBarMenu = document.getElementById(burger.dataset.target);

    // Ajouter un événement au clic pour le bouton burger
    burger.addEventListener('click', () => {
        // Basculer la classe "is-active" sur le burger et le menu
        burger.classList.toggle('is-active');
        navBarMenu.classList.toggle('is-active');
    });

    new window.AjaxRequest();
});

document.addEventListener("DOMContentLoaded", () => {
    document.querySelectorAll(".debug_value").forEach((element) => {
        element.addEventListener("click", (event) => {
            const selection = window.getSelection()?.toString(); // Récupérer le texte sélectionné
            if (selection && selection.trim() !== "") {
                return; // Ne rien faire si du texte est sélectionné
            }
            element.classList.toggle("expanded"); // Toggle si aucune sélection
        });
    });
});
