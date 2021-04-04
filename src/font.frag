# version 330 core

out vec4 colour;
in vec2 tex_coords;

uniform sampler2D tex;

void main() {
    colour = vec4(1.0, 1.0, 1.0, texture(tex, tex_coords).r);

    if (colour.a == 0.0) {
        discard;
    }
}
