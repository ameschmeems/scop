#version 330 core

in VS_OUTPUT
{
	vec3 Color;
	vec2 TexCoord;
} IN;

out vec4 Color;

uniform sampler2D myTexture;

void main()
{
	// Color = vec4(IN.Color, 1.0f);
	// Color = texture(myTexture, IN.TexCoord) * vec4(IN.Color, 1.0);
	Color = texture(myTexture, IN.TexCoord);
}