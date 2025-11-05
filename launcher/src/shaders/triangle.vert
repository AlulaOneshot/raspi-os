#version 330 core
layout(location = 0) in vec3 position;
layout(location = 1) in vec3 normal;
layout(location = 2) in vec2 tex_coords;

out vec2 frag_tex_coords;
out vec3 frag_normal;

uniform mat4 transform;

void main() {
    gl_Position = transform * vec4(position, 1.0);
    frag_tex_coords = tex_coords;
    frag_normal = normal;
}