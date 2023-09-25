import {GameOptions, Gloam} from "gloam-engine";
import redSpr from "./art/red.png?url";
import appleSpr from "./art/apple.png?url";
import {GameObject, GloamScene, GloamWrapper} from "./GameObject.ts";
import {Textures} from "./gloam/Texture.ts";
import * as keyboardjs from "keyboardjs";


export async function start() {
    await Textures.load_texture('apple', appleSpr);
    await Textures.load_texture('red', redSpr);

    const ref = Gloam.start(new GameOptions(512, 512, 2, 0xFFFFFF));
    GloamWrapper.scene = new GloamScene(ref);

    ref.add_object(new Snake());

}
class Apple extends GameObject
{
    pos_x = 0;
    pos_y = 0;
    tex: number;
    init(): void
    {
        this.tex = Textures.get_tex("apple");
        this.move();
    }

    update(delta: number): void
    {
        Gloam.draw_texture(1, this.pos_x * 32, this.pos_y * 32);
    }

    public move()
    {
        // TODO rand
        this.pos_x = (Math.round(Math.random() * 16));
        this.pos_y = (Math.round(Math.random() * 16));
    }
}

export class Snake extends GameObject
{
    segments: [number, number][] = [[14, 13], [13, 13], [12, 13]];
    x_dir = 1;
    y_dir = 0;

    next_x = 0;
    next_y = 0;
    apple: Apple

    mps = 0;
    private tex: number;

    init(): void
    {
        this.tex = Textures.get_tex("red");
        this.apple = this.scene.add_object(new Apple());


        keyboardjs.bind('a', () => {
            this.next_x = -1;
            this.next_y = 0;
        });
        keyboardjs.bind('d', () => {
            this.next_x = 1;
            this.next_y = 0;
        });
        keyboardjs.bind('w', () => {
            this.next_x = 0;
            this.next_y = -1;
        });
        keyboardjs.bind('s', () => {
            this.next_x = 0;
            this.next_y = 1;
        });
    }

    update(delta: number): void
    {
        this.mps += delta;

        if (this.mps > 0.5 * (3 / this.segments.length))
        {
            if ((this.next_x + this.x_dir != 0 || this.next_y + this.y_dir != 0)
                && (this.next_x != 0 || this.next_y != 0))
            {
                this.x_dir = this.next_x;
                this.y_dir = this.next_y;
                this.next_x = 0;
                this.next_y = 0;
            }

            this.mps = 0;
            // console.log(this.segments);
            // move the head
            const head = this.segments[0];
            this.segments.unshift([head[0] + this.x_dir, head[1] + this.y_dir]);

            console.log(head[0], this.apple.pos_x);
            // if collecting an apple, don't do this.
            if (this.segments[0][0] == this.apple.pos_x && this.segments[0][1] == this.apple.pos_y)
            {
                this.apple.move();
            } else
            {
                this.segments.pop();
            }
        }

        for (let segment of this.segments)
        {
            // console.log("Draw", segment)
            Gloam.draw_texture(this.tex, segment[0] * 32, segment[1] * 32);
        }
    }
}