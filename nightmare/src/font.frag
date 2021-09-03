# version 330 core

out vec4 colour;
in vec2 tex_coords;
in vec4 tex_rect;

uniform sampler2D tex;

void main() {
    vec2 the_final_coord = tex_rect.xy + tex_coords * tex_rect.zw;

    colour = vec4(1.0, 1.0, 1.0, texture(tex, the_final_coord).r);

    if (colour.a == 0.0) {
        discard;
    }
}
