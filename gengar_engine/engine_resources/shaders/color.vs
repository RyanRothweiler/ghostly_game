layout (location = 0) in vec3 aPos;
  
out vec4 oColor;

uniform mat4 projection;
uniform mat4 model;
uniform mat4 view;
uniform vec4 color;

void main()
{
    oColor = color;
    gl_Position = projection * view * model * vec4(aPos, 1.0);
}