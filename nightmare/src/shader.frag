#version 330 core

in vec2 tex_coords;

// Texture region
in vec4 tex_rect;
// in vec2 tex_pos;
// in vec2 tex_size;

out vec4 colour;

uniform sampler2D tex;

void main() {
    // vec2 coords = fract(tex_coords);
    // vec2 the_final_coord = tex_pos + tex_coords * tex_size;
    vec2 the_final_coord = tex_rect.xy + tex_coords * tex_rect.zw;

    colour = texture(tex, the_final_coord);

    if (colour.a == 0.0) {
        discard;
    }
}
