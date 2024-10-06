layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 aTexCoord;
layout (location = 2) in vec3 aNormal;
layout (location = 3) in vec3 aTangent;
layout (location = 4) in vec3 aBiTangent;
  
out vec2 vTexCoord;
out vec3 vFragPos;
out vec3 vViewPos;
out vec3 vLightPos;
out vec3 vLightColor;

out vec3 vNormal;
out vec3 vNormalTan;
out vec3 vNormalBiTan;

uniform vec3 viewPos;
uniform vec3 lightPos;
uniform vec3 lightColor;
uniform mat4 projection;
uniform mat4 model;
uniform mat4 view;

void main()
{
    vTexCoord = aTexCoord;
    vViewPos = viewPos;
    vLightPos = lightPos;
    vLightColor = lightColor;
    vFragPos = vec3(model * vec4(aPos, 1.0));

    vNormal =       normalize(mat3(transpose(inverse(model))) * aNormal);
    vNormalTan =    normalize(mat3(transpose(inverse(model))) * aTangent);
    vNormalBiTan =  normalize(mat3(transpose(inverse(model))) * aBiTangent);

    gl_Position = projection * view * model * vec4(aPos, 1.0);
}