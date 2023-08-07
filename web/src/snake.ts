import {Gloam, JsGameObject} from "gloam-engine";
import redSpr from "./art/red.png?url";
import appleSpr from "./art/apple.png?url";
import {bindKey} from '@rwh/keystrokes'

class Apple implements JsGameObject
{
    pos_x = 0;
    pos_y = 0;

    init(): void
    {
        // console.log("appl init");
        this.move();
    }

    update(delta: number): void
    {
        // console.log("update apple", delta);
        let meee;
        Gloam.with_object(2, (other) => {
            Gloam.with_object(2, (other) => {
                // console.log("This is illegal... or not?");
            })
        });

        // console.log(meee);

        Gloam.with_objects([1], (a) => {
            // console.log("test")
            // console.log(a);
        });

        Gloam.with_type("Snake", (other: Snake) => {
            // console.log("SNAME: ", other.apple)
        });

        Gloam.draw_texture(1, this.pos_x * 32, this.pos_y * 32);
    }

    public move()
    {
        // TODO rand
        this.pos_x = (Math.round(Math.random() * 16));
        this.pos_y = (Math.round(Math.random() * 16));
    }
}

export class Snake implements JsGameObject
{
    segments: [number, number][] = [[14, 13], [13, 13], [12, 13]];
    x_dir = 1;
    y_dir = 0;

    next_x = 0;
    next_y = 0;
    apple: Apple

    mps = 0;

    init(): void
    {
        // console.log(import.meta.url)
        const url = new URL(redSpr, import.meta.url);
        // console.log(url);
        Gloam.load_texture(url.href);
        Gloam.load_texture(new URL(appleSpr, import.meta.url).href);
        this.apple = new Apple();
        Gloam.add_object(this.apple);

        bindKey('a', () => {
            this.next_x = -1;
            this.next_y = 0;
        });
        bindKey('d', () => {
            this.next_x = 1;
            this.next_y = 0;
        });
        bindKey('w', () => {
            this.next_x = 0;
            this.next_y = -1;
        });
        bindKey('s', () => {
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
            Gloam.draw_texture(0, segment[0] * 32, segment[1] * 32);
        }
    }
}