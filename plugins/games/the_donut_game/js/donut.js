export class Donut {
    constructor() {
        this.button = document.getElementById('donut_button');
        this.countElement = document.getElementById('donut_count');
        this.count = parseInt(this.countElement.textContent, 10) || 0;
        this.updateScheduled = false;
        
        this.initializeEvents();
    }

    initializeEvents() {
        this.button.addEventListener('click', () => this.handleClick());
    }

    handleClick() {
        this.count++;
        if (!this.updateScheduled) {
            this.updateScheduled = true;
            requestAnimationFrame(() => this.updateDisplay());
            this.checkLevel();
        }
    }

    checkLevel() {
        if(this.count === 5) {
            this.countElement.classList.add('donut_count_level_1');
            this.countElement.parentElement.classList.add('donut_square_level_1');
        }
    }

    updateDisplay() {
        this.countElement.textContent = this.count;
        this.updateScheduled = false;
    }

    getCount() {
        return this.count;
    }
}
