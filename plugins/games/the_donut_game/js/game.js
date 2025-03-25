import { Donut } from './donut.js';
import { Incubator } from './incubator.js';

// Game Controller
export class Game {
    constructor() {
        this.donut = new Donut();
        this.incubator = new Incubator();
        
        this.initializeGameLoop();
    }

    initializeGameLoop() {
        setInterval(() => this.checkGameProgress(), 100);
    }

    checkGameProgress() {
        const count = this.donut.getCount();
        
        if(count === 10) {
            this.incubator.upgrade(1);
        }
    }
}