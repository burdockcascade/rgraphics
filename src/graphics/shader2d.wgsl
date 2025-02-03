struct VertexInput {
  @location(0) position: vec3<f32>,
  @location(1) uv: vec2<f32>,
};

struct VertexOutput {
  @builtin(position) position: vec4<f32>,
  @location(1) uv: vec2<f32>,
};

struct DrawUniforms {
  transform: mat4x4<f32>,
  color: vec4<f32>
};

@group(0) @binding(0)
var<uniform> draw_uniforms: DrawUniforms;

@group(1) @binding(0)
var t_diffuse: texture_2d<f32>;

@group(1) @binding(1)
var s_diffuse: sampler;

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
  var output: VertexOutput;
  output.position = draw_uniforms.transform * vec4<f32>(input.position, 1.0);
  output.uv = input.uv;
  return output;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return draw_uniforms.color + textureSample(t_diffuse, s_diffuse, in.uv);
}