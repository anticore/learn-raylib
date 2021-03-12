#version 330

in vec2 fragTexCoord;
in vec4 fragColor;

out vec4 out_color;

uniform float time;
uniform vec2 resolution;
uniform sampler2D lastframe;

void main() {
    vec2 uv = vec2(gl_FragCoord.x / resolution.x, gl_FragCoord.y / resolution.y);
    vec2 uvv = uv;
    uv -= 0.5;
    uv /= vec2(resolution.y / resolution.x, 1);

    vec4 lf = texture2D(lastframe, uvv);
    uv += vec2(sin(time), cos(time));
    vec4 c = 
        fract(uv.x * 10.) > 0.5 && fract(uv.y * 10.) > 0.5 
            ? vec4(1) 
            : vec4(0);

    float f = 0.1;
    out_color = c * (1.-f) + lf * f;
}