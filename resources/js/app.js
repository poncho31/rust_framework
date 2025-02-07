import { Modal }       from './models/modal.js';
import { Tooltip }     from './models/tooltip.js';
// import { AjaxRequest } from './models/ajax_request.js';
import { Ajax }        from './models/ajax.js';
import { DesktopControls} from './models/desktop_controls.js';


window.Modal       = Modal;
window.Tooltip     = Tooltip;
// window.AjaxRequest = AjaxRequest;
window.Ajax        = Ajax;


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



document.addEventListener('DOMContentLoaded', () => {
    new Tooltip('.has-tooltip');
});


document.querySelectorAll('.file-input').forEach((input) => {
    input.addEventListener('change', (event) => {
      const fileName = event.target.files.length
        ? Array.from(event.target.files).map((file) => file.name).join(', ')
        : 'Aucun fichier sélectionné';
      input.closest('.file').querySelector('.file-name').textContent = fileName;
    });
  });
  

        
document.addEventListener('DOMContentLoaded', () => {
    window.desktop_controls = new DesktopControls();
});