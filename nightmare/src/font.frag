# version 330 core

in vec2 tex_coords;
in vec4 tex_rect;
// in float the_red;

out vec4 colour;

uniform sampler2D tex;

void main() {
    vec2 the_final_coord = tex_rect.st + tex_coords * tex_rect.pq;
    float alpha = texture(tex, the_final_coord).r;
    colour = vec4(1.0, 0.7, 0.2, alpha);
}
