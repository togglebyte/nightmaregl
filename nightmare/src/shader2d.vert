# version 330 core

layout (location = 0) in vec3 position;
layout (location = 1) in vec2 uv_coords;
layout (location = 3) in mat4 model;

// Texture region
layout (location = 7) in vec4 _tex_rect;

// View projection
uniform mat4 vp;
uniform float red;

out vec2 tex_coords;
out vec4 tex_rect;
// out float the_red;

void main() {
    vec3 _position = position;
    _position.y += red;
    gl_Position = vp * model * vec4(_position, 1.0);
    tex_coords = uv_coords;
    tex_rect = _tex_rect;
    // the_red = 1.0;
}
