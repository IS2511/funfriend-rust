#version 330 core
in vec2 TexCoord;

uniform sampler2D texture1;

out vec4 FragColor;

void main() {
  vec4 tex = texture(texture1, TexCoord);
  FragColor = vec4(mix(vec3(0.0, 0.0, 0.0), tex.rgb, tex.a), 1.0);
}