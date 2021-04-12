# version 330 core

out vec4 colour;
in vec2 tex_coords;

// Sprite size as sprite.size / texture.size.
// e.g a 32x32 texture and a 64x32 sprite results
// in a sprite size of 2,1
in vec2 sprite_size;

/* int vec2 */ 

uniform sampler2D tex;

void main() {
    // Offset N tiles
    // In a 2x2 tileset 0.5, 0.0 would 
    // pick the second tile,
    // 0.0, 0.5 would pick the third etc.
    vec2 offset = vec2(0.5, 0.5);

    // Size of a tile in relation to the 
    // texture, where 0,0 to 1,1 is the 
    // entire texture.
    vec2 tile_size = vec2(0.5, 0.5);

    vec2 tex_coords = fract(sprite_size * tex_coords) * tile_size + offset;
    colour = texture(tex, tex_coords);

    if (colour.a == 0.0) {
        discard;
    }
}
