#version 330 core

#define MAX_LIGHTS 16

out vec4 FragColor;

in vec3 Normal;
in vec3 FragPos;

uniform vec3 lightCount;
uniform vec3 lightPos[MAX_LIGHTS];
uniform vec3 lightColor[MAX_LIGHTS];
uniform vec3 objectColor;

void main()
{
    // ambient
    float ambientStrength = 0.1;
    vec3 ambient;
    vec3 diffuse;
    vec3 norm = normalize(Normal);
    for (int i=0; i<MAX_LIGHTS; ++i) {
        ambient += ambientStrength * lightColor[i];
        vec3 lightDir = normalize(lightPos[i] - FragPos);
        float diff = max(dot(norm, lightDir), 0.0);
        diffuse += diff * lightColor[i];
    }
    vec3 result = (ambient + diffuse) * objectColor;

    FragColor = vec4(result, 1.0);
}
