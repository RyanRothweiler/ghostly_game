precision highp float;

in vec2 vTexCoord;
in vec3 vNormal;
in vec3 vFragPos;
in vec3 vViewPos;
in vec3 vLightPos;

in vec3 vNormalTan;
in vec3 vNormalBiTan;

out vec4 FragColor;
  
uniform sampler2D tex;
uniform sampler2D normalTex;

void main()
{
    float specularStrength = 1.5;
    vec3 lightColor = vec3(1, 1, 1);

    // normal map
    mat3 tbn = mat3(vNormalTan, vNormalBiTan, vNormal);

    vec3 texNormal = texture(normalTex, vTexCoord).rgb;
    texNormal = (texNormal * 2.0) - 1.0;
    vec3 norm = normalize(tbn * texNormal);

    // Calculations
    vec3 lightDir = normalize(vLightPos - vFragPos);

    // Specular
    vec3 viewDir = normalize(vViewPos - vFragPos);
    vec3 reflectDir = reflect(-lightDir, norm);  

    float spec = pow(max(dot(viewDir, reflectDir), 0.0), 32.0);
    vec3 specular = specularStrength * spec * lightColor;  

    // Diffuse
    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = diff * lightColor * vec3(texture(tex, vTexCoord));

    vec3 result = (diffuse + specular);
    FragColor = vec4(result, 1.0);

    // Gamma correction    
    float gamma = 2.2;
    FragColor.rgb = pow(FragColor.rgb, vec3(1.0 / gamma));

} 