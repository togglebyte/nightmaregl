#version 330 core

layout (location = 0) in vec3 position;
layout (location = 1) in vec2 uv_coords;

layout (location = 3) in mat4 transform;
layout (location = 10) in vec2 _tex_pos;
layout (location = 11) in vec2 _tex_size;
layout (location = 12) in vec2 _tile_count;

uniform mat4 vp;
uniform sampler2D tex;
uniform float pixel_scale;

out vec2 tex_coords;
out vec2 tex_pos;
out vec2 tex_size;
out vec2 tile_count;

void main() {

    mat4 scaling_matrix = mat4(1.0);
    scaling_matrix[0][0] = pixel_scale;
    scaling_matrix[1][1] = pixel_scale;

    gl_Position = vp * scaling_matrix * transform * vec4(position, 1.0);

    // Off to the fragment shader you go
    tex_pos = _tex_pos;
    tex_size = _tex_size;
    /* tile_count = vec2(3.0, 1.0); */
    tile_count = _tile_count;
    tex_coords = uv_coords * tile_count;
}
