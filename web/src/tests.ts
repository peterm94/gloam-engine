import {Gloam} from "gloam-engine";
import {GameObject} from "./GameObject.ts";


export class A extends GameObject {

    init(): void {
        Gloam.register("TEST", {
            trigger() {
                console.log("hello");
            }
        })
    }

    update(delta: number): void {
        // Gloam.draw_circle_filled(100, 100, 40, 0x0000FF);
        // Gloam.draw_rectangle(400, 30, 24, 144, 41, 0xFF0020);
        let txt = "yhello does a newline work?\nNyes?\nyNo?";
        // let txt = "a;";
        let dims = Gloam.measure_text(txt, 40);
        Gloam.draw_rectangle_filled(0, 0, dims.width, dims.height, 0x333e4f);
        // draw_rectangle(x, baseline - size.offset_y, size.width, size.height, BLUE);

        Gloam.draw_text(txt, 0, dims.offset_y, 40, 0x1ebd48);
        Gloam.trigger("TEST")
    }
}