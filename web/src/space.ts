import {Gloam, JsGameObject} from "../../dist";

export class SpaceInvaders implements JsGameObject {

    player = new Player();
    aliens: Alien[] = [];

    init(): void {
        Gloam.add_object(this.player);

        for (let i = 0; i < 5; i++) {
            const alien = new Alien();
            Gloam.add_object(alien);
            this.aliens.push(alien)
        }
    }

    update(delta: number): void {

    }
}

class Alien implements JsGameObject {
    init(): void {
    }

    update(delta: number): void {
    }
}

class Player implements JsGameObject {
    init(): void {
    }

    update(delta: number): void {
    }
}

class AlienControl implements JsGameObject {
    init(): void {
    }

    update(delta: number): void {
    }
}