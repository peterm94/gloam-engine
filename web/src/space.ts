import {GameObject, Scene} from "./GameObject.ts";
import alien from "./art/aliens.png?url";
import {Textures} from "./Texture";
import {Gloam} from "gloam-engine";

export async function start() {
    await Textures.load_texture("alien", alien)

    const scene = new Scene();

    Gloam.register_scene(scene);

    scene.add_object(new SpaceInvaders());
}


export class SpaceInvaders extends GameObject {
    player: Player
    controller: AlienControl

    init(): void {
        this.controller = this.scene.add_object(new AlienControl());
        this.player = this.scene.add_object(new Player());
    }

    update(delta: number): void {
        console.log("hello")
    }
}

class Alien extends GameObject {
    alien_tex: number

    constructor(parent: GameObject) {
        super(parent);
        this.alien_tex = Textures.get_tex("alien");
    }

    init(): void {
    }

    update(delta: number): void {
        Gloam.draw_texture(this.alien_tex, 0, 0);
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
            const alien = this.scene.add_object(new Alien(this));
            this.aliens.push(alien)
        }
    }

    update(delta: number): void {
    }
}