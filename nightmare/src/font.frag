# version 330 core

in vec2 tex_coords;
in vec4 tex_rect;

out vec4 colour;

uniform sampler2D tex;
uniform vec3 col;

void main() {
    vec2 the_final_coord = tex_rect.st + tex_coords * tex_rect.pq;
    vec4 alpha = vec4(1.0, 1.0, 1.0, texture(tex, the_final_coord).r);
    colour = vec4(col, 1.0) * alpha;
}
