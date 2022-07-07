uniform float uTime;

in vec4 col;

layout (location = 0) out vec4 outColor;

void main()
{
    float pct = abs(sin(uTime * 0.5));
    outColor = mix(vec4(col.rgb, 1.0), vec4(0.0, 0.0, 1.0, 1.0), pct);
}