precision highp float;

in vec2 vTexCoord;
in vec3 vNormal;
in vec3 vFragPos;

out vec4 FragColor;
  
uniform sampler2D tex;

void main()
{
    vec3 lightPos = vec3(2, 2, 0);
    vec3 lightColor = vec3(1, 0, 0);

    vec3 norm = normalize(vNormal);
    vec3 lightDir = normalize(lightPos - vFragPos);

    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = diff * lightColor;

    vec3 result = diffuse * vec3(texture(tex, vTexCoord));
    FragColor = vec4(result, 1.0);
} 