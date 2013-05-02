#pragma version(1)
#pragma rs java_package_name(android.renderscript.cts)

typedef struct fmax_f32_3_in {
    float3 first;
    float3 second;
} input;

void root(const input* in, float3* out){
    *out = fmax(in->first, in->second);
}