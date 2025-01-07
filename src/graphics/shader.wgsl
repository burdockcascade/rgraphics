struct VertexInput {
  @location(0) position: vec3<f32>,
  @location(1) color: vec3<f32>,
  @location(2) uv: vec2<f32>,
};

struct VertexOutput {
  @builtin(position) position: vec4<f32>,
  @location(0) color: vec3<f32>,
  @location(1) uv: vec2<f32>,
};

@group(0) @binding(0)
var<uniform> transform: mat4x4<f32>; 

//@group(0) @binding(1)
//var t_diffuse: texture_2d<f32>;
//
//@group(0) @binding(2)
//var s_diffuse: sampler;

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
  var out: VertexOutput;
  out.position = transform * vec4<f32>(in.position, 1.0); 
  out.color = in.color;
  out.uv = in.uv;
  return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
  //var texture_color = textureSample(t_diffuse, s_diffuse, in.uv);
  //return texture_color * vec4<f32>(in.color, 1.0);
    return vec4<f32>(in.color, 1.0); // Add an alpha value of 1.0

}