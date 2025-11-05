#version 330 core
out vec4 FragColor;

in vec2 frag_tex_coords;
in vec3 frag_normal;

uniform sampler2D texture_0;

void main()
{
    FragColor = texture(texture_0, frag_tex_coords) * vec4(frag_normal, 1.0);
}