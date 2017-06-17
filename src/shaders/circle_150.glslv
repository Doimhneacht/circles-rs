#version 150 core

in vec2 a_Pos;
in float a_Radius;
in float a_Time;
in vec4 a_BaseColor;
in vec4 a_NewColor;

out VertexData {
    float radius;
    vec4 color;
} VertexOut;

void main() {
    gl_Position = vec4(a_Pos, 0, 1);
    VertexOut.radius = a_Radius;
    vec4 color_diff = a_NewColor - a_BaseColor;
    VertexOut.color = a_BaseColor + color_diff * a_Time;
}
