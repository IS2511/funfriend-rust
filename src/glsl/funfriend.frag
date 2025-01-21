#version 330 core
#define PI 3.141592653589793
in vec2 TexCoord;

uniform sampler2D texture1;
uniform vec2 funfriendSize;
uniform vec2 resolution;
uniform float time;

out vec4 FragColor;

vec2 rotate(vec2 uv, float angle) {
  // algebra formula for rotation by matrix , https://en.wikipedia.org/wiki/Rotation_matrix
  mat2 m = mat2(cos(angle), -sin(angle), sin(angle), cos(angle));
  // rotation of uv with matrix algebra formula where is set the rotation angle
  return m * uv;
}

vec3 rotateY(vec3 uv, float angle) {
  mat3 m = mat3(
    cos(angle),  0.0, sin(angle),
    0.0,         1.0, 0.0,
    -sin(angle), 0.0, cos(angle)
  );
  return m * uv;
}

void main() {
  vec2 uv = TexCoord;

  // [0.0 - 1.0] -> [-0.5 - 0.5]
  uv -= 0.5;

  // BUDDYBOUNCE-Y
  float y = sin(time * PI);
  uv.y += y * 0.05;
  uv = rotateY(vec3(uv, 0.0), y * (20. / 360.) * PI*2.).xy;

  // BUDDYBOUNCE-X
  float x = sin(time * 0.5 * PI);
  uv.x += x * 0.05;
  uv = rotate(uv, x * (5. / 360.) * PI*2.);

  // scale funfriend to fit in the center
  vec2 scale = funfriendSize / resolution;
  uv /= scale;

  // [-0.5 - 0.5] -> [0.0 - 1.0]
  uv += 0.5;

  // ooo you're a vflipper... you like flipping your y coordinate.... ooooo
  uv.y = 1.0 - uv.y;

  vec4 texColor = texture(texture1, uv);
  
//  FragColor = vec4(texColor.a, texColor.a, texColor.a, 1.0);
  FragColor = texColor;
}