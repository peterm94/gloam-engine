import './style.css'
import {Gloam, load_game} from "gloam-engine";
import {Tests} from "./tests.ts";
import {GameObject, GameObjectTest} from "./GameObject.ts";
import {SpaceInvaders} from "./space.ts";

document.querySelector<HTMLDivElement>('#app')!.innerHTML = `
  <div>
    <canvas id="glcanvas" width="1000" height="1000" tabindex='1' hidden></canvas>
  </div>
`

function run() {
    document.getElementById("glcanvas").removeAttribute("hidden");
    document.getElementById("glcanvas").focus();
    load_game(() => {
        // Gloam.add_object(new Tests());
        new SpaceInvaders();
    });
}

run();