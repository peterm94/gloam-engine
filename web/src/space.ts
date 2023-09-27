import {GameObject, GloamScene, GloamWrapper} from "./GameObject.ts";
import alien from "./art/aliens.png?url";
import player from "./art/player.png?url";
import {Textures} from "./gloam/Texture.ts";
import {GameOptions, Gloam} from "gloam-engine";
import * as keyboardjs from "keyboardjs";
import {MathUtil} from "./gloam/Util.ts";
import {Sprite, SpriteSheet} from "./gloam/SpriteSheet";


export async function start()
{
    await Textures.load_texture("alien", alien)
    await Textures.load_texture("player", player)

    const ref = Gloam.start(new GameOptions(256, 256, 0x05092f));
    GloamWrapper.scene = new GloamScene(ref);

    ref.add_object(new SpaceInvaders());
}


export class SpaceInvaders extends GameObject
{
    player: Player
    controller: AlienControl

    last_frames = Array(1000).fill(0.001);
    col = 0;

    init(): void
    {
        this.controller = this.scene.add_object(new AlienControl());
        this.player = this.scene.add_object(new Player());
    }

    update(delta: number): void
    {

        this.last_frames.push(delta);
        this.last_frames.shift();

        let avg = this.last_frames.reduce((a, b) => a + b) / this.last_frames.length;
        this.col += 100;
        Gloam.draw_rectangle_filled(0, 0, 55, 20, 0xc2c7cf)
        Gloam.draw_text(`\n${Math.trunc(1000 / avg / 1000)}fps`, 0, 0, 20, this.col++, true);
    }
}

class Alien extends GameObject
{
    private readonly alien_sprites: SpriteSheet;
    s1: Sprite
    s2: Sprite

    timer: number = 0

    constructor(parent: GameObject)
    {
        super(parent);

        this.alien_sprites = new SpriteSheet(Textures.get_tex("alien"), 16, 16);
        this.s1 = this.alien_sprites.texture(0, 0);
        this.s2 = this.alien_sprites.texture(1, 0);
    }

    init(): void
    {
    }

    update(delta: number): void
    {
        const x = Math.floor(this.id() / 10);
        const y = this.id() % 10;

        this.timer += delta;
        if (Math.trunc(this.timer) % 2 == 0)
        {
            this.s1.draw(x * 16, y * 16);
        } else
        {
            this.s2.draw(x * 16, y * 16);
        }
    }
}

class Player extends GameObject
{
    private player_tex: number;

    x = 128;
    move_left = false;
    move_right = false;
    shoot = false;
    shoot_cdr = 0;

    constructor()
    {
        super();
        this.player_tex = Textures.get_tex("player");
    }

    init(): void
    {

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

    update(delta: number): void
    {

        // Move
        if (this.move_left === true)
        {
            this.x -= delta * 100;
        }

        if (this.move_right === true)
        {
            this.x += delta * 100;
        }

        this.x = MathUtil.clamp(this.x, 10, 256 - 10 - 16);

        // Update shoot timer
        this.shoot_cdr -= delta;

        // Shoot
        if (this.shoot && this.shoot_cdr < 0)
        {
            this.shoot = false
            this.scene.add_object(new Bullet(this.x + 7, 230));
        }


        Gloam.draw_texture(this.player_tex, this.x, 230);

    }
}

class Bullet extends GameObject
{

    constructor(public x: number, public y: number)
    {
        super();
    }

    init(): void
    {
    }

    update(delta: number): void
    {
        Gloam.draw_rectangle_filled(this.x, this.y, 2, 7, 0xFFFFFF);

        this.y -= delta * 200;
    }

}

class AlienControl extends GameObject
{

    aliens: Alien[] = [];

    init(): void
    {
        console.log("adding 100 entities")
        for (let i = 0; i < 100; i++)
        {
            const alien = this.scene.add_object(new Alien(this));
            this.aliens.push(alien)
        }
    }

    update(delta: number): void
    {
        // console.log(delta * 1000    );
    }
}