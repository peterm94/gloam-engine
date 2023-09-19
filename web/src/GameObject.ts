import {Scene, Transform} from "gloam-engine"

export abstract class GameObject {
    private static id_count = 0;

    _id: number = ++GameObject.id_count;
    transform: Transform
    parent: number

    id(): number {
        return this._id;
    }

    destroy() {
        Scene.remove_object(this.id());
    }

    constructor(parent?: GameObject) {
        const parent_id = (parent === undefined) ? 0 : parent.id();
        Scene.add_child(parent_id, this);
    }

    abstract init(): void;

    abstract update(delta: number): void;
}

class A extends GameObject {
    init(): void {
    }

    update(delta: number): void {
    }
}