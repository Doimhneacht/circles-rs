#version 150 core

in VertexData {
    vec4 color;
    vec2 uv;
} VertexIn;

out vec4 Target0;

void main() {
    float r = dot(VertexIn.uv, VertexIn.uv);
    float d = 0.75;
    float alpha = 1;
    if (r > 1) {
        alpha = 0;
    } else if (r > d) {
        alpha = 1 - (r - d) / (1 - d);
    }

    Target0 = vec4(VertexIn.color.rgb, VertexIn.color.a * alpha);
}
