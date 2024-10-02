layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 aTexCoord;
layout (location = 2) in vec3 aNormal;
layout (location = 2) in vec3 aTangent;
layout (location = 2) in vec3 aBiTangent;
  
out vec2 vTexCoord;
out vec3 vFragPos;
out vec3 vViewPos;
out vec3 vLightPos;
out mat3 vTBN;

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
    vFragPos = vec3(model * vec4(aPos, 1.0));

    //vNormal = normalize(mat3(transpose(inverse(model))) * aNormal);

    vec3 T = normalize(vec3(model * vec4(aTangent,   0.0)));
    vec3 B = normalize(vec3(model * vec4(aBiTangent, 0.0)));
    vec3 N = normalize(vec3(model * vec4(aNormal,    0.0)));
    vTBN = mat3(T, B, N);

    gl_Position = projection * view * model * vec4(aPos, 1.0);
}