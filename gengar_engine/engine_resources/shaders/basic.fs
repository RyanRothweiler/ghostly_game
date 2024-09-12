precision highp float;

in vec2 texCoord;
in vec3 normal;
in vec3 fragPos;

out vec4 FragColor;
  
uniform sampler2D tex;

void main()
{
    vec3 lightPos = vec3(5, 0, 0);
    vec3 lightColor = vec3(1, 0, 0);

    vec3 norm = normalize(normal);
    vec3 lightDir = normalize(lightPos - fragPos);

    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = diff * lightColor;

    vec3 result = diffuse * vec3(texture(tex, texCoord));
    FragColor = vec4(result, 1.0);
} 