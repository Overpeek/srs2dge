struct VertexInput {
	@location(0) pos: vec2<f32>,
	@location(1) uv: vec2<f32>,
	@location(2) col: vec4<f32>,
};

struct FragmentInput {
	@builtin(position) pos: vec4<f32>,
	@location(0) col: vec4<f32>,
	@location(1) uv: vec2<f32>,
};

struct UniformInput {
	mvp: mat4x4<f32>,
};

@group(0)
@binding(0)
var<uniform> ubo: UniformInput;

@group(0)
@binding(1)
var t_texture: texture_2d<f32>;

@group(0)
@binding(2)
var s_texture: sampler;

@vertex
fn vs_main(vin: VertexInput) -> FragmentInput {
	var fin: FragmentInput;
	fin.pos = ubo.mvp * vec4<f32>(vin.pos, 0.0, 1.0);
	fin.col = vin.col;
	fin.uv = vin.uv;
	return fin;
}

@fragment
fn fs_main(fin: FragmentInput) -> @location(0) vec4<f32> {
	let aa = 0.01;
	let val = textureSample(t_texture, s_texture, fin.uv).x;

	// border
	let alpha = smoothstep(0.35 - aa, 0.35 + aa, val);

	// outline
	let col = vec3<f32>(smoothstep(0.5 - aa, 0.5 + aa, val));
	return fin.col * vec4<f32>(col, alpha);
}

