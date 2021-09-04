# version 330 core

layout (location = 0) in vec3 position;
layout (location = 1) in vec2 uv_coords;
layout (location = 3) in mat4 model;

// Texture region
layout (location = 7) in vec4 _tex_rect;

// View projection
uniform mat4 vp;
uniform float rred;

out vec2 tex_coords;
out vec4 tex_rect;
out float red;

void main() {
    mat4 _model = model;
    _model[3][2] += rred;
    gl_Position = vp * _model * vec4(position, 1.0);
    tex_coords = uv_coords;
    tex_rect = _tex_rect;
    red = rred;
}
