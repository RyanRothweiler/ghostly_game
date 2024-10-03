precision highp float;

in vec2 vTexCoord;
in vec3 vFragPos;
in vec3 vViewPos;
in vec3 vLightPos;
in mat3 vTBN;

in vec3 vTangentLightPos;
in vec3 vTangentViewPos;
in vec3 vTangentFragPos;

out vec4 FragColor;
  
uniform sampler2D tex;
uniform sampler2D normalTex;

void main()
{
    float specularStrength = 1.5;
    vec3 lightColor = vec3(1, 1, 1);

    // obtain normal from normal map in range [0,1]
    vec3 normal = texture(normalTex, vTexCoord).rgb;

    // transform normal vector to range [-1,1]
    normal = normalize(normal * 2.0 - 1.0);  // this normal is in tangent space
   
    // get diffuse color
    //vec3 color = texture(tex, vTexCoord).rgb;
    vec3 color = vec3(1, 1, 1);

    // ambient
    vec3 ambient = 0.1 * color;

    // diffuse
    vec3 lightDir = normalize(vTangentLightPos - vTangentFragPos);
    float diff = max(dot(lightDir, normal), 0.0);
    vec3 diffuse = diff * color;

    // specular
    vec3 viewDir = normalize(vTangentViewPos - vTangentFragPos);
    vec3 reflectDir = reflect(-lightDir, normal);
    vec3 halfwayDir = normalize(lightDir + viewDir);  
    float spec = pow(max(dot(normal, halfwayDir), 0.0), 32.0);

    vec3 specular = vec3(0.2) * spec;
    FragColor = vec4(ambient + diffuse + specular, 1.0);

} 