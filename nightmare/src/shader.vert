#version 330 core

layout (location = 0) in vec3 position;
layout (location = 1) in vec2 uv_coords;
layout (location = 3) in mat4 nonsense;

// Texture region
layout (location = 7) in vec4 _tex_rect;
// layout (location = 7) in vec2 _tex_pos;
// layout (location = 8) in vec2 _tex_size;

uniform mat4 vp;

out vec2 tex_coords;
out vec4 tex_rect;
// out vec2 tex_pos;
// out vec2 tex_size;

void main() {
    gl_Position = vp * nonsense * vec4(position, 1.0);
    tex_coords = uv_coords;
    // tex_pos = _tex_pos;
    // tex_size = _tex_size;
    tex_rect = _tex_rect;
}
