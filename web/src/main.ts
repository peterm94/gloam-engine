import './style.css'
import {load_game} from "gloam-engine";
import {start} from "./space.ts";

document.querySelector<HTMLDivElement>('#app')!.innerHTML = `
  <div>
    <canvas id="glcanvas" width="256" height="256" tabindex='1' hidden></canvas>
  </div>
`

function run() {
    document.getElementById("glcanvas").removeAttribute("hidden");
    document.getElementById("glcanvas").focus();
    load_game(() => {
        // Gloam.add_object(new Tests());
        start();
    });
}

run();