Texture2D tex0 : register(t0);
SamplerState samp0 : register(s0);

struct VS_OUT {
    float4 pos : SV_Position;
    float4 color : COLOR;
    float2 uv : TEXCOORD;
};

float4 main(VS_OUT input) : SV_Target {
    float4 tex_color = tex0.Sample(samp0, input.uv);
    float out_alpha = tex_color.a * input.color.a;
    clip(out_alpha - (4.0 / 255.0));
    return float4(tex_color.rgb * input.color.rgb, out_alpha);
}
