#version 330 core
layout(location = 0) in vec3 aPos;
layout(location = 1) in vec3 normal;
layout(location = 2) in vec2 texCoords;

out vec2 fragTexCoords;
out vec3 fragNormal;

uniform mat4 transform;

void main() {
    gl_Position = transform * vec4(aPos, 1.0);
    fragTexCoords = texCoords;
    fragNormal = normal;
}