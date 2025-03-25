
export class Incubator {
    constructor() {
        this.element = document.getElementById('incubator');
    }

    upgrade(level) {
        switch(level) {
            case 1:
                this.element.classList.add('incubator_level_1');
                this.element.textContent = 'Incubator Level 1';
                break;
            // Add more levels here as needed
        }
    }
}
