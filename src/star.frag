uniform float time;
uniform vec3 uColor;
uniform vec2 uIResolution;

layout (location = 0) out vec4 outColor;

float dist(vec2 p0, vec2 pf)
{
    return sqrt((pf.x - p0.x) * (pf.x - p0.x) + (pf.y - p0.y) * (pf.y - p0.y));
}

void main()
{
    float d = dist(uIResolution.xy * 0.5, gl_FragCoord.xy) * (sin(time) + 2.0) * 0.003;
    outColor = mix(vec4(uColor.x * gl_FragCoord.x, uColor.xy, 1.0), vec4(uColor, 1.0), d);
}