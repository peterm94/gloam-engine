import {GameObject, GloamScene, GloamWrapper} from "./GameObject.ts";
import alien from "./art/aliens.png?url";
import {Textures} from "./Texture";
import {Gloam} from "gloam-engine";

export async function start() {
    await Textures.load_texture("alien", alien)

    const ref = Gloam.start();
    GloamWrapper.scene = new GloamScene(ref);

    ref.add_object(new SpaceInvaders());
}


export class SpaceInvaders extends GameObject {
    player: Player
    controller: AlienControl

    frame = 0;
    fps = 0;
    last_frames = Array(1000).fill(0.001);

    init(): void {
        this.controller = this.scene.add_object(new AlienControl());
        this.player = this.scene.add_object(new Player());
    }

    update(delta: number): void {

        this.last_frames.push(delta);
        this.last_frames.shift();

        let avg = this.last_frames.reduce((a, b) => a + b) / this.last_frames.length;

        Gloam.draw_text(`${Math.trunc(1000 / avg / 1000)}fps`, 100, 100, 100, 0xFFFFFF);

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
        for (let i = 0; i < 10000; i++) {
            const alien = this.scene.add_object(new Alien(this));
            this.aliens.push(alien)
        }
    }

    update(delta: number): void {
        console.log(delta * 1000    );
    }
}