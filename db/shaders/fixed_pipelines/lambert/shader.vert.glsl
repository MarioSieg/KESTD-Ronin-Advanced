#version 450

layout(location = 0) in vec4 a_Pos;
layout(location = 1) in vec2 a_TexCoord;
layout(location = 0) out vec2 v_TexCoord;

layout(push_constant) uniform pushContants {
    mat4 u_World;
    mat4 u_ViewProjection;
};

void main() {
    v_TexCoord = a_TexCoord;
    gl_Position = (u_World * u_ViewProjection) * a_Pos;
}