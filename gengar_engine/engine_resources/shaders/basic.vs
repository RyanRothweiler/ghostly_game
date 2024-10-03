layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 aTexCoord;
layout (location = 2) in vec3 aNormal;
layout (location = 3) in vec3 aTangent;
  
out vec2 vTexCoord;
out vec3 vFragPos;
out vec3 vViewPos;
out vec3 vLightPos;
out mat3 vTBN;

out vec3 vTangentLightPos;
out vec3 vTangentViewPos;
out vec3 vTangentFragPos;

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
    
    mat3 normalMatrix = transpose(inverse(mat3(model)));
    vec3 T = normalize(normalMatrix * aTangent);
    vec3 N = normalize(normalMatrix * aNormal);
    T = normalize(T - dot(T, N) * N);
    vec3 B = cross(N, T);

    mat3 TBN = transpose(mat3(T, B, N));    
    vTangentLightPos = TBN * lightPos;
    vTangentViewPos  = TBN * viewPos;
    vTangentFragPos  = TBN * vFragPos;

    gl_Position = projection * view * model * vec4(aPos, 1.0);
}