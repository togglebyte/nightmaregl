#version 330 core

in vec2 tex_coords;

// Texture region
in vec4 tex_rect;

out vec4 colour;

uniform sampler2D tex;

void main() {
    vec2 the_final_coord = tex_rect.xy + tex_coords * tex_rect.zw;

    colour = texture(tex, the_final_coord);

    if (colour.a == 0.0) {
        discard;
    }
}
