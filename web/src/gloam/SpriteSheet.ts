import {Gloam} from "../../../dist";

export class Sprite
{
    constructor(readonly tex_id: number, readonly x: number, readonly y: number, readonly w: number, readonly h: number)
    {
    }

    draw(x: number, y: number) {
        Gloam.draw_texture_part(this.tex_id, x, y, this.x, this.y, this.w, this.h);
    }
}

/**
 * Convenient way to load multiple sprites from a single Sprite Sheet.
 */
export class SpriteSheet
{
    private readonly texture_width: number
    private readonly texture_height: number

    /**
     * Create a new SpriteSheet.
     * @param tex_id Texture ID.
     * @param resource The base sprite sheet resource.
     * @param tileWidth The width of the tiles on the sheet.
     * @param tileHeight The height of the tiles on the sheet.
     */
    constructor(private readonly tex_id: number, private readonly tileWidth: number, private readonly tileHeight: number)
    {
        const tex_info = Gloam.texture_info(tex_id);
        this.texture_width = tex_info.width;
        this.texture_height = tex_info.height;
        this.tileWidth = tileWidth;
        this.tileHeight = tileHeight;
    }

    /**
     * Get a texture from the SpriteSheet.
     * @param column The column index for the texture.
     * @param row The row index for the texture.
     * @param width Optional override for the texture width.
     * @param height Optional override for the texture height.
     */
    texture(column: number, row: number, width?: number, height?: number): Sprite
    {
        const w = width || this.tileWidth;
        const h = height || this.tileHeight;

        return new Sprite(this.tex_id, column * this.tileWidth, row * this.tileHeight, w, h);
    }

    /**
     * Get a texture from the spritesheet using pixel offsets.
     * @param x X Pixel offset.
     * @param y Y Pixel offset.
     * @param width Width in pixels.
     * @param height Height in pixels.
     */
    textureFromPoints(x: number, y: number, width: number, height: number): Sprite
    {
        return new Sprite(this.tex_id, x, y, width, height);
    }

    /**
     * Create a texture by index.
     * @param index Tile index of the texture to load.
     * @returns The loaded texture.
     */
    // TODO this is busted?
    textureFromIndex(index: number): Sprite
    {
        const col = index % (this.texture_width / this.tileWidth);
        const row = Math.floor(index / (this.texture_height / this.tileHeight));

        return new Sprite(this.tex_id, col * this.tileWidth,
            row * this.tileHeight,
            this.tileWidth, this.tileHeight);
    }

    /**
     * Get multiple textures from the SpriteSheet.
     * @param frames Desired texture indexes from the SpriteSheet. Supplied as pairs of [column, row].
     * @param width Optional override for the texture width.
     * @param height Optional override for the texture height.
     * @returns The loaded textures.
     */
    textures(frames: [number, number][], width?: number, height?: number): Sprite[]
    {
        const textures = [];
        for (const frame of frames)
        {
            textures.push(this.texture(frame[0], frame[1], width, height));
        }
        return textures;
    }

    /**
     * Slices a row of textures with. Starting at [start] and ending at [end], inclusively.
     * @param row The row of textures to slice.
     * @param start The start index of the slice. Inclusive.
     * @param end The end index of the slice. Inclusive.
     * @param width Optional override for the texture width.
     * @param height Optional override for the texture height.
     * @returns The loaded texture.
     */
    textureSliceFromRow(row: number, start: number, end: number, width?: number, height?: number): Sprite[]
    {
        const textures = [];

        for (let i = start; i <= end; i++)
        {
            textures.push(this.texture(i, row, width, height));
        }

        return textures;
    }

    /**
     * Slices all sprites out of the first row of the SpriteSheet.
     * @returns PIXI.Texture[] The loaded textures.
     */
    textureSliceFromSheet(): Sprite[]
    {
        const end = Math.floor(this.texture_width / this.tileWidth);
        return this.textureSliceFromRow(0, 0, end - 1)
    }
}
