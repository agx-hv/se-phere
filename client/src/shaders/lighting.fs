#version 330 core

#define MAX_LIGHTS 16

out vec4 FragColor;

in vec3 Normal;
in vec3 FragPos;
in vec2 TexCoords;

uniform vec3 lightCount;
uniform vec3 lightPos[MAX_LIGHTS];
uniform vec3 lightColor[MAX_LIGHTS];
uniform vec3 objectColor;

uniform sampler2D ourTexture;

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

    FragColor = vec4(result, 1.0);// *texture(ourTexture, FragPos);

    // vec3 ambient = vec3(0.1, 0.1, 0.1); // Ambient lighting
    // vec3 lightDir = normalize(vec3(0.0,  1.0, 0.0)); // Example light direction
    // vec3 diffuse = texture(ourTexture, TexCoords).rgb; // Sample texture
    // vec3 normal = normalize(Normal);
    // float diff = max(dot(normal, lightDir), 0.0);
    // vec3 lighting = ambient + diff * diffuse; // Lambertian lighting model

    // FragColor = vec4(lighting, 1.0);
}
