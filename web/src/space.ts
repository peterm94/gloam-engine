import {GameObject} from "./GameObject.ts";
import alien from "./art/aliens.png?url";
import {Textures} from "./Texture";
import {Gloam} from "gloam-engine";
export async function start()
{
    await Textures.load_texture("alien", alien)

    new SpaceInvaders();
}

export class SpaceInvaders extends GameObject
{
    player: Player
    controller: AlienControl

    init(): void
    {
        this.controller = new AlienControl();
        this.player = new Player();
    }

    update(delta: number): void
    {
        console.log("hello")
    }
}

class Alien extends GameObject
{
    alien_tex: number
    constructor(parent: GameObject)
    {
        super(parent);
        this.alien_tex = Textures.get_tex("alien");
    }

    init(): void
    {
    }

    update(delta: number): void
    {
        Gloam.draw_texture(this.alien_tex, 0, 0);
    }
}

class Player extends GameObject
{
    init(): void
    {
    }

    update(delta: number): void
    {
    }
}

class AlienControl extends GameObject
{

    aliens: Alien[] = [];

    init(): void
    {
        for (let i = 0; i < 5; i++)
        {
            const alien = new Alien(this);
            this.aliens.push(alien)
        }
    }

    update(delta: number): void
    {
    }
}