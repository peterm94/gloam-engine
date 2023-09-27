import {GameObject} from "../GameObject.ts";
import {Sprite} from "./SpriteSheet.ts";

export interface SpriteAnimation {
    id: number;
    sprites: Sprite[];
}

export class AnimatedSpriteController extends GameObject {
    private current_state: number;

    private readonly animation_states: Map<number, SpriteAnimation> = new Map();

    constructor(private initial_state: number, animations: SpriteAnimation[]) {
        super();

        this.current_state = initial_state;
        animations.forEach(value => this.animation_states.set(value.id, value));
    }
    init(): void {
    }

    update(delta: number): void {
    }


    public set_animation(state_id: number, reset = false): void {
        if (this.current_state === state_id && !reset) return;

        const animation = this.animation_states.get(state_id);

        // Apply

    }
}