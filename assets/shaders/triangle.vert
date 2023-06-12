#version 330 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec3 Normal;
layout (location = 2) in vec2 TexCoord;
layout (location = 3) in vec3 Color;

out VS_OUTPUT
{
	flat vec3 Color;
	vec2 TexCoord;
} OUT;

// uniform mat4 transform;
uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main()
{
	// gl_Position = vec4(Position, 1.0);
	// gl_Position = transform * vec4(Position, 1.0);
	gl_Position = projection * view * model * vec4(Position, 1.0);
	OUT.Color = Color;
	// OUT.TexCoord = TexCoord;
}