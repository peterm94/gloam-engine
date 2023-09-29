import {GameObject} from "../GameObject.ts";
import {Sprite} from "./SpriteSheet.ts";

export interface SpriteAnimation {
    id: number;
    sprites: Sprite[];
    config?: AnimationConfig
    events?: { [key: number]: () => void };
}

export interface AnimationConfig {
    animation_speed?: number;
    end_action?: "STOP" | "REVERSE" | "LOOP"
    end_event?: () => void;
}

export class AnimatedSpriteController extends GameObject {
    // Current frame
    private frame_index = 0;

    // Direction to update the frame
    private frame_advancer = 1;

    // Sprite to be drawn
    private current_animation: SpriteAnimation;

    // Timer stuff
    private elapsed = 0;
    private next_trigger_time = -1;
    private trigger_interval = -1;

    private readonly animation_states: Map<number, SpriteAnimation> = new Map();

    constructor(initial_state: number, animations: SpriteAnimation[]) {
        super();

        animations.forEach(value => this.animation_states.set(value.id, value));
        this.current_animation = animations[initial_state];
        this.set_animation(initial_state, true);
    }

    init(): void {
    }

    update(delta: number): void {
        this.elapsed += delta;
        if (this.next_trigger_time === -1) {
            // Init state
            this.next_trigger_time = this.elapsed + this.trigger_interval;
        } else if (this.elapsed > this.next_trigger_time) {
            // do the thing
            let last_frame = false;
            let next_frame = this.frame_index + this.frame_advancer;
            this.next_trigger_time += this.trigger_interval;

            if (next_frame === -1 || next_frame === this.current_animation.sprites.length) {
                last_frame = true;
                switch (this.current_animation?.config?.end_action || "LOOP") {
                    case "STOP":
                        next_frame = this.frame_index;
                        this.frame_advancer = 0;
                        break;
                    case "REVERSE":
                        this.frame_advancer *= -1;
                        next_frame += this.frame_advancer * 2;
                        break;
                    case "LOOP":
                        next_frame %= this.current_animation.sprites.length;
                        break;
                }
            }

            this.frame_index = next_frame;
            this.current_animation?.events?.[this.frame_index]?.call(this);

            if (last_frame) {
                this.current_animation?.config?.end_event?.call(this);
            }
        }
    }

    public draw(x: number, y:number) {
        this.current_animation.sprites[this.frame_index].draw(x, y);
    }


    public set_animation(state_id: number, reset = false): void {
        if (this.current_animation.id === state_id && !reset) return;

        this.current_animation = this.animation_states.get(state_id);
        this.trigger_interval = this.current_animation.config.animation_speed;
        this.next_trigger_time = -1;
        this.frame_index = 0;
    }
}