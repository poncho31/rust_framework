class Tooltip {
    constructor(selector) {
        this.tooltips = document.querySelectorAll(selector);
        console.log(selector);
        this.init();
    }

    init() {
        this.tooltips.forEach((tooltip) => {
            const tooltipText = tooltip.getAttribute('data-tooltip');
            const tooltipElement = this.createTooltipElement(tooltipText);
            tooltip.appendChild(tooltipElement);

            tooltip.addEventListener('mouseenter', () => this.showTooltip(tooltip, tooltipElement));
            tooltip.addEventListener('mouseleave', () => this.hideTooltip(tooltipElement));
        });
    }

    createTooltipElement(text) {
        const tooltipElement = document.createElement('div');
        tooltipElement.className = 'tooltip-content';
        tooltipElement.textContent = text;
        return tooltipElement;
    }

    showTooltip(parent, tooltipElement) {
        tooltipElement.style.opacity = '1';
        tooltipElement.style.visibility = 'visible';
        tooltipElement.style.top = `${parent.offsetHeight + 5}px`;
        tooltipElement.style.left = `50%`;
        tooltipElement.style.transform = `translateX(-50%)`;

        // Ajuste la position si le tooltip dépasse les bords
        const tooltipRect = tooltipElement.getBoundingClientRect();
        if (tooltipRect.right > window.innerWidth) {
            tooltipElement.style.left = `auto`;
            tooltipElement.style.right = `0`;
            tooltipElement.style.transform = `none`;
        }
        if (tooltipRect.left < 0) {
            tooltipElement.style.left = `0`;
            tooltipElement.style.transform = `none`;
        }
    }

    hideTooltip(tooltipElement) {
        tooltipElement.style.opacity = '0';
        tooltipElement.style.visibility = 'hidden';
    }
}


// Exporter la classe Modal
export { Tooltip };
