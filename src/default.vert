#version 330 core

layout (location = 0) in vec3 position;
layout (location = 1) in vec2 uv_coords;

layout (location = 3) in mat4 transform;
layout (location = 10) in vec2 tex_offset;
layout (location = 11) in vec2 tex_scale;

uniform mat4 vp;
uniform sampler2D tex;
uniform float pixel_scale;

out vec2 tex_coords;
out float pixel_scale_f;

void main() {
    mat4 scaling_matrix = mat4(1.0);
    scaling_matrix[0][0] = pixel_scale;
    scaling_matrix[1][1] = pixel_scale;

    gl_Position = vp * scaling_matrix * transform * vec4(position, 1.0);

    tex_coords = tex_offset + uv_coords * tex_scale;
    pixel_scale_f = pixel_scale;
}
