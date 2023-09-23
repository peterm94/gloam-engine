import {GloamWasm, Transform} from "gloam-engine"

interface Scene {
    add_object<T extends GameObject>(object: T): T;

    remove_object(object_id: number): void;
}

export class DumbScene implements Scene {
    add_object<T extends GameObject>(object: T): T {
        throw new Error("You need to add the object to the scene before you can use it.")
    }

    remove_object(object_id: number): void {
        throw new Error("You need to add the object to the scene before you can use it.")
    }
}

export class GloamWrapper {
    static scene: Scene = new DumbScene();
}

export class GloamScene implements Scene {

    constructor(private ref: GloamWasm) {

    }

    add_object<T extends GameObject>(object: T): T {
        object.scene = this;
        this.ref.add_object(object);
        return object;
    }

    remove_object(object_id: number): void {
        this.ref.remove_object(object_id);
    }
}

export abstract class GameObject {
    private static id_count = 0;

    _id: number = ++GameObject.id_count;
    transform: Transform = new Transform();
    parent: number
    scene: Scene = GloamWrapper.scene;

    id(): number {
        return this._id;
    }

    destroy() {
        this.scene.remove_object(this._id);
    }

    constructor(parent?: GameObject) {
        const parent_id = (parent === undefined) ? 0 : parent.id();
        // Scene.add_child(parent_id, this);
    }

    abstract init(): void;

    abstract update(delta: number): void;
}