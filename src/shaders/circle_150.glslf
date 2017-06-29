#version 150 core

in VertexData {
    vec4 color;
    vec2 uv;
    float radius;
} VertexIn;

out vec4 Target0;

void main() {
    float r = dot(VertexIn.uv, VertexIn.uv);
    float rd = 10;
    float d = (VertexIn.radius - rd) / VertexIn.radius;

    float alpha = 1;
    if (r > 1) {
        alpha = 0;
    } else if (r > d) {
        alpha = 1 - (r - d) / (1 - d);
    }

    Target0 = vec4(VertexIn.color.rgb, VertexIn.color.a * alpha);
}
