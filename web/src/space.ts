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
    col = 0;

    init(): void {
        this.controller = this.scene.add_object(new AlienControl());
        this.player = this.scene.add_object(new Player());
    }

    update(delta: number): void {

        this.last_frames.push(delta);
        this.last_frames.shift();

        let avg = this.last_frames.reduce((a, b) => a + b) / this.last_frames.length;
        this.col += 100;
        Gloam.draw_text(`${Math.trunc(1000 / avg / 1000)}fps`, 100, 100, 100, this.col++);
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
        const x = Math.floor(this.id() / 10);
        const y = this.id() % 10;
        Gloam.draw_texture(this.alien_tex, x * 16, y * 16);
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
        console.log("adding 10000 entities")
        for (let i = 0; i < 100; i++) {
            const alien = this.scene.add_object(new Alien(this));
            this.aliens.push(alien)
        }
    }

    update(delta: number): void {
        // console.log(delta * 1000    );
    }
}