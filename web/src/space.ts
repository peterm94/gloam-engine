import {GameObject} from "./GameObject.ts";

export class SpaceInvaders extends GameObject {

    player: Player
    controller: AlienControl

    init(): void {
        this.controller = new AlienControl();
        this.player = new Player();
    }

    update(delta: number): void {
    }
}

class Alien extends GameObject {
    constructor(parent: GameObject)
    {
        super(parent);
    }
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

    aliens: Alien[] = [];
    
    init(): void {
        for (let i = 0; i < 5; i++) {
            const alien = new Alien(this);
            this.aliens.push(alien)
        }
    }

    update(delta: number): void {
    }
}