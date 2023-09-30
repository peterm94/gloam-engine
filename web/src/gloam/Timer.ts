import {GameObject} from "./GameObject.ts";

export class Timer extends GameObject {

    current_time: number = 0;

    constructor(public interval: number, public on_trigger: () => void, public repeat: boolean = false) {
        super();
    }

    init(): void {
    }

    update(delta: number): void {
        this.current_time += delta;

        if (this.current_time > this.interval) {
            this.on_trigger();
            if (this.repeat) {
                this.current_time = 0;
            } else {
                this.destroy();
            }
        }

    }

}