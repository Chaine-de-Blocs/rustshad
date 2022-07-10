uniform float uTime;
uniform vec3 cameraPosition;

in vec4 col;
in vec3 nor;
in vec3 pos;

layout (location = 0) out vec4 outColor;

void main()
{
    float pct = abs(sin(uTime * 0.5));
    vec4 color = mix(vec4(col.rgb, 1.0), vec4(0.0, 0.0, 0.0, 1.0), pct);
    outColor = vec4(calculate_light(vec3(0.0, 0.0, 0.8), cameraPosition, color.rgb, cameraPosition, normalize(nor), 0.8, 0.2), 1.0);
}