import {GameObject, GloamScene, GloamWrapper} from "./GameObject.ts";
import alien from "./art/aliens.png?url";
import player from "./art/player.png?url";
import {Textures} from "./gloam/Texture.ts";
import {GameOptions, Gloam} from "gloam-engine";
import * as keyboardjs from "keyboardjs";
import {MathUtil} from "./gloam/Util.ts";
import {SpriteSheet} from "./gloam/SpriteSheet";
import {AnimatedSpriteController} from "./gloam/AnimatedSpriteController.ts";


export async function start() {
    await Textures.load_texture("alien", alien)
    await Textures.load_texture("player", player)

    const ref = Gloam.start(new GameOptions(256, 256, 0x05092f));
    GloamWrapper.scene = new GloamScene(ref);

    ref.add_object(new SpaceInvaders());
}


export class SpaceInvaders extends GameObject {
    player: Player
    controller: AlienControl

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
        Gloam.draw_rectangle_filled(0, 0, 55, 20, 0xc2c7cf)
        Gloam.draw_text(`\n${Math.trunc(1000 / avg / 1000)}fps`, 0, 0, 20, this.col++, true);
    }
}

class Alien extends GameObject {
    private readonly animation: AnimatedSpriteController

    constructor(parent: GameObject) {
        super(parent);

        const alien_sprites = new SpriteSheet(Textures.get_tex("alien"), 16, 16);
        this.animation = this.scene.add_object(new AnimatedSpriteController(0, [{
            id: 0,
            sprites: [alien_sprites.texture(0, 0), alien_sprites.texture(1, 0)],
            config: {
                end_action: "LOOP",
                animation_speed: 0.5
            }
        }]));
    }

    init(): void {
    }

    update(delta: number): void {
        const x = Math.floor(this.id() / 10);
        const y = this.id() % 10;

        this.animation.draw(x * 16, y * 16);
    }
}

class Player extends GameObject {
    private player_tex: number;

    x = 128;
    move_left = false;
    move_right = false;
    shoot = false;
    shoot_cdr = 0;

    constructor() {
        super();
        this.player_tex = Textures.get_tex("player");
    }

    init(): void {

        keyboardjs.bind('a', event => {
                this.move_left = true;
            },
            event => {
                this.move_left = false;
            })
        keyboardjs.bind('d', event => {
                this.move_right = true;
            },
            event => {
                this.move_right = false;
            })

        keyboardjs.bind('space', event => {
            this.shoot = true;
        })
    }

    update(delta: number): void {

        // Move
        if (this.move_left === true) {
            this.x -= delta * 100;
        }

        if (this.move_right === true) {
            this.x += delta * 100;
        }

        this.x = MathUtil.clamp(this.x, 10, 256 - 10 - 16);

        // Update shoot timer
        this.shoot_cdr -= delta;

        // Shoot
        if (this.shoot && this.shoot_cdr < 0) {
            this.shoot = false
            this.scene.add_object(new Bullet(this.x + 7, 230));
        }


        Gloam.draw_texture(this.player_tex, this.x, 230);

    }
}

class Bullet extends GameObject {

    constructor(public x: number, public y: number) {
        super();
    }

    init(): void {
    }

    update(delta: number): void {
        Gloam.draw_rectangle_filled(this.x, this.y, 2, 7, 0xFFFFFF);

        this.y -= delta * 200;
    }

}

class AlienControl extends GameObject {

    aliens: Alien[] = [];

    init(): void {
        console.log("adding 100 entities")
        for (let i = 0; i < 100; i++) {
            const alien = this.scene.add_object(new Alien(this));
            this.aliens.push(alien)
        }
    }

    update(delta: number): void {
        // console.log(delta * 1000    );
    }
}