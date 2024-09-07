precision highp float;

in vec2 TexCoord;

out vec4 FragColor;
  
void main()
{
    FragColor = vec4(TexCoord.x, 1, 0, 1);
} 