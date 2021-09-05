# version 330 core

// uniform vec2 u_resolution = vec2(1224.0, 180.0);

in vec2 tex_coords;

out vec4 omg;

uniform sampler2D tex;
uniform vec3 col;

void main() {
    // vec2 u_resolution = vec2(1224.0, 180.0);
	// vec2 st = gl_FragCoord.xy / u_resolution;
	// gl_FragColor = vec4(st.x,st.y,0.0,1.0);


    vec4 t = texture(tex, tex_coords);
    t.r = col.r;
    omg = t;
}



// in vec2 tex_coords;
// in vec4 tex_rect;

// out vec4 colour;

// uniform sampler2D tex;
// uniform vec3 col;

// void main() {
//     vec2 the_final_coord = tex_rect.st + tex_coords * tex_rect.pq;
//     vec4 alpha = vec4(1.0, 1.0, 1.0, texture(tex, the_final_coord).r);
//     colour = alpha;
//     // colour = vec4(col, 1.0) * alpha;
// }
