layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 aTexCoord;
layout (location = 2) in vec3 aNormal;
  
out vec4 vertexColor;
out vec2 texCoord;
out vec3 normal;
out vec3 fragPos;

uniform mat4 projection;
uniform mat4 model;
uniform mat4 view;

void main()
{
    texCoord = aTexCoord;
    normal = aNormal;

    fragPos = vec3(model * vec4(aPos, 1.0));
    gl_Position = projection * view * model * vec4(aPos, 1.0);
}