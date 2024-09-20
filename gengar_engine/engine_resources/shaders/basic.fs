precision highp float;

in vec2 vTexCoord;
in vec3 vNormal;
in vec3 vFragPos;
in vec3 vViewPos;

out vec4 FragColor;
  
uniform sampler2D tex;

void main()
{
    float specularStrength = 1.5;

    vec3 lightPos = vec3(2, 2, 0);
    vec3 lightColor = vec3(1, 1, 1);

    // Calculations
    vec3 norm = normalize(vNormal);
    vec3 lightDir = normalize(lightPos - vFragPos);

    // Specular
    vec3 viewDir = normalize(vViewPos - vFragPos);
    vec3 reflectDir = reflect(-lightDir, norm);  

    float spec = pow(max(dot(viewDir, reflectDir), 0.0), 32.0);
    vec3 specular = specularStrength * spec * lightColor;  

    // Diffuse
    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = diff * lightColor;

    vec3 result = (diffuse + specular) * vec3(texture(tex, vTexCoord));
    FragColor = vec4(result, 1.0);
} 