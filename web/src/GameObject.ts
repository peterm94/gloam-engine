import {NextFrame, Scene as GloamScene, Transform} from "gloam-engine"


interface SceneWrapper {
    add_object<T extends GameObject>(object: T): T;
    remove_object(object_id: number): void;

    update(delta: number): void;
}

export class DumbScene implements SceneWrapper {
    add_object<T extends GameObject>(object: T): T {
        throw new Error("You need to add the object to the scene before you can use it.")
    }

    update(delta: number) {
        throw new Error("Don't use this")
    }

    remove_object(object_id: number): void {
        throw new Error("You need to add the object to the scene before you can use it.")
    }
}
export class Scene implements SceneWrapper {
    scene: GloamScene = new GloamScene();
    next_frame: NextFrame = new NextFrame();

    add_object<T extends GameObject>(object: T): T {
        this.next_frame.add_child(0, object);
        object.scene = this;
        return object;
    }

    update(delta: number) {
        const temp = this.next_frame;
        // TODO we probably only need to trigger this if we have actually done something.
        // TODO we can probably split out add/remove and only call if we have batched stuff.
        this.next_frame = new NextFrame();
        this.scene.update(temp, delta);
    }

    remove_object(object_id: number): void {
        this.next_frame.remove_object(object_id);
    }
}

export abstract class GameObject {
    private static id_count = 0;

    _id: number = ++GameObject.id_count;
    transform: Transform = new Transform();
    parent: number
    scene: SceneWrapper = new DumbScene();

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

class A extends GameObject {
    init(): void {
    }

    update(delta: number): void {
    }
}