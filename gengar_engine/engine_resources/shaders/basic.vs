layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 aTexCoord;
layout (location = 2) in vec3 aNormal;
  
out vec2 vTexCoord;
out vec3 vNormal;
out vec3 vFragPos;
out vec3 vViewPos;
out vec3 vLightPos;

uniform vec3 viewPos;
uniform vec3 lightPos;
uniform mat4 projection;
uniform mat4 model;
uniform mat4 view;

void main()
{
    vTexCoord = aTexCoord;
    vViewPos = viewPos;
    vLightPos = lightPos;
    vNormal = normalize(mat3(transpose(inverse(model))) * aNormal);
    vFragPos = vec3(model * vec4(aPos, 1.0));

    gl_Position = projection * view * model * vec4(aPos, 1.0);
}