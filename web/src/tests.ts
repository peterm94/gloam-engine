import {Gloam, JsGameObject} from "gloam-engine";


class A implements JsGameObject
{

    constructor(readonly name: String)
    {
    }

    init(): void
    {
        Gloam.register("TEST", {trigger()
            {
                console.log("hello");
            }})
    }

    update(delta: number): void
    {
        // Gloam.draw_circle_filled(100, 100, 40, 0x0000FF);
        // Gloam.draw_rectangle(400, 30, 24, 144, 41, 0xFF0020);
        let txt = "yhello does a newline work?\nNyes?\nyNo?";
        // let txt = "a;";
        let dims = Gloam.measure_text(txt, 40);
        Gloam.draw_rectangle_filled(50, 50 - dims.offset_y, dims.width, dims.height, 0x333e4f);
        // draw_rectangle(x, baseline - size.offset_y, size.width, size.height, BLUE);

        Gloam.draw_text(txt, 50, 50, 40, 0x1ebd48);
        Gloam.trigger("TEST")
    }
}

class B implements JsGameObject
{
    init(): void
    {
    }

    update(delta: number): void
    {
    }

}

export class Tests implements JsGameObject
{
    private first: number;
    private second: number;

    init(): void
    {
        console.log("init")
        this.first = Gloam.add_object(new A("first"));
        this.second = Gloam.add_object(new A("second"));
        Gloam.add_object(new A("third"));
        Gloam.add_object(new B());
        Gloam.add_object(new B());
        Gloam.add_object(new B());
    }

    update(delta: number): void
    {
        Gloam.draw_line(10, 10, 100, 10, 10, 0);
        Gloam.with_objects([this.first, this.second], objects => {
            const [a, b]: [A, A] = objects;

            Gloam.with_objects([this.first, this.second], obk => {
                // console.log(obk)
            });
        });
    }

}