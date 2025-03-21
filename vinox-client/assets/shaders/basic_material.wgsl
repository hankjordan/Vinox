#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::pbr_types
#import bevy_pbr::mesh_bindings

struct BasicMaterial {
    color: vec4<f32>,
};

struct Vertex {
#ifdef VERTEX_POSITIONS
    @location(0) position: vec3<f32>,
#endif
#ifdef VERTEX_NORMALS
    @location(1) normal: vec3<f32>,
#endif
#ifdef VERTEX_UVS
    @location(2) uv: vec2<f32>,
#endif
#ifdef VERTEX_TANGENTS
    @location(3) tangent: vec4<f32>,
#endif
#ifdef VERTEX_COLORS
    @location(4) color: vec4<f32>,
#endif
#ifdef SKINNED
    @location(5) joint_indices: vec4<u32>,
    @location(6) joint_weights: vec4<f32>,
#endif
};

@group(1) @binding(0)
var<uniform> material: BasicMaterial;
@group(1) @binding(1)
var base_color_texture: texture_2d<f32>;
@group(1) @binding(2)
var base_color_sampler: sampler;

struct FragmentInput {
    #import bevy_pbr::mesh_vertex_output
}

@fragment
fn fragment(
in: FragmentInput,
) -> @location(0) vec4<f32> {
    var return_color = material.color * textureSample(base_color_texture, base_color_sampler, in.uv);
    #ifdef VERTEX_COLORS
        return_color = return_color * in.color;
    #endif

    return return_color;

}