layout (location = 0) in vec3 aPos;
layout (location = 1) in vec4 aColor;
  
out vec4 color;

uniform mat4 projection;
uniform mat4 model;
uniform mat4 view;

void main()
{
    color = aColor;
    gl_Position = projection * view * model * vec4(aPos, 1.0);
}