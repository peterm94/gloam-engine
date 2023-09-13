import {GameObject} from "./GameObject.ts";

export class SpaceInvaders extends GameObject {

    player = new Player();
    aliens: Alien[] = [];

    init(): void {
        for (let i = 0; i < 5; i++) {
            const alien = new Alien();
            this.aliens.push(alien)
        }
    }

    update(delta: number): void {
    }
}

class Alien extends GameObject {
    init(): void {
    }

    update(delta: number): void {
    }
}

class Player extends GameObject {
    init(): void {
    }

    update(delta: number): void {
    }
}

class AlienControl extends GameObject {
    init(): void {
    }

    update(delta: number): void {
    }
}