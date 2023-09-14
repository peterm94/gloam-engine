import {Gloam} from 'gloam-engine';

export class Textures
{
    static tex_map: Map<string, number> = new Map<string, number>();

    public static async load_texture(name: string, url: string): Promise<number>
    {
        const tex_id = await Gloam.load_texture(new URL(url, import.meta.url).href)

        Textures.tex_map.set(name, tex_id);
        return tex_id;
    }

    public static get_tex(name: string): number
    {
        return Textures.tex_map.get(name);
    }
}