# version 330 core

out vec4 colour;

in vec2 tex_coords;
in vec2 tex_pos;
in vec2 tex_size;
in vec2 tile_count;

uniform sampler2D tex;

void main() {
    // wonderful intrets says:  and now you want tex_pos + tex_coords * tex_size
    vec2 coords = fract(tex_coords);
    vec2 the_final_coord = tex_pos + coords * tex_size;
    colour = texture(tex, the_final_coord);

    if (colour.a == 0.0) {
        discard;
    }
}
