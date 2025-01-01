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