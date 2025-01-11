float2 position register(t0);

float4 VSMain() {
    return float4(position.xy, 0.0, 1.0);
}