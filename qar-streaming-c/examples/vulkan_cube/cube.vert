#version 450
layout(push_constant) uniform Camera { mat4 mvp; } camera;
layout(location = 0) out vec3 color;
const vec3 corners[8] = vec3[](
    vec3(-1,-1,-1), vec3(1,-1,-1), vec3(1,1,-1), vec3(-1,1,-1),
    vec3(-1,-1,1), vec3(1,-1,1), vec3(1,1,1), vec3(-1,1,1));
const int indices[36] = int[](
    0,2,1,0,3,2, 4,5,6,4,6,7, 0,1,5,0,5,4,
    3,7,6,3,6,2, 0,4,7,0,7,3, 1,2,6,1,6,5);
const vec3 colors[6] = vec3[](
    vec3(1,.2,.2), vec3(.2,1,.2), vec3(.2,.2,1),
    vec3(1,1,.2), vec3(1,.2,1), vec3(.2,1,1));
void main() {
    gl_Position = camera.mvp * vec4(corners[indices[gl_VertexIndex]], 1);
    color = colors[gl_VertexIndex / 6];
}
