#version 330 core
layout (location = 0) in vec3 aPos;
  
out vec4 vertexColor;

uniform mat4 model;
uniform mat4 view;

void main()
{
    gl_Position = view * model * vec4(aPos, 1.0);
}