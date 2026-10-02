struct Uniforms {
    scale: f32,
    aspect_ratio: f32,
    camera_translation: vec2<f32>,
    window_size: vec2<f32>,
    rotation_angle: f32,
    _padding: f32,
    rotation_center: vec2<f32>,
};
@group(0) @binding(0) var<uniform> uniforms: Uniforms;

struct Particle {
    pos: vec2<f32>,
    vel: vec2<f32>,
};

@group(1) @binding(0) var<storage, read> particles: array<Particle>;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

struct FragmentInput {
    @location(0) color: vec4<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    let particle = particles[vertex_index];
    let translated_pos = particle.pos - uniforms.camera_translation;
    let rotation_center_translated = uniforms.rotation_center - uniforms.camera_translation;
    let pos_relative_to_center = translated_pos - rotation_center_translated;
    let rotated_pos = rotate2d(pos_relative_to_center, uniforms.rotation_angle);
    let final_pos = rotated_pos + rotation_center_translated;
    let camera_pos = final_pos * uniforms.scale;

    var ndc: vec2<f32>;
    ndc = camera_pos / uniforms.window_size * 2.0;

    var output: VertexOutput;
    output.position = vec4<f32>(ndc, 0.0, 1.0);
    let speed = length(particle.vel);
    let hue = 0.3 * log(max(speed, 1e-6));
    output.color = vec4<f32>(hsv_to_rgb(vec3<f32>(hue, 1.0, 1.0)), 0.1);
    return output;
}

@fragment
fn fs_main(input: FragmentInput) -> @location(0) vec4<f32> {
    return input.color;
}

fn rotate2d(pos: vec2<f32>, angle: f32) -> vec2<f32> {
    let cos_angle = cos(angle);
    let sin_angle = sin(angle);
    return vec2<f32>(
        pos.x * cos_angle - pos.y * sin_angle,
        pos.x * sin_angle + pos.y * cos_angle
    );
}

fn modulo(a: f32, b: f32) -> f32 {
    return a - b * floor(a / b);
}

fn hsv_to_rgb(hsv: vec3<f32>) -> vec3<f32> {
    let h = fract(hsv.x);
    let s = hsv.y;
    let v = hsv.z;

    let c = v * s;
    let x = c * (1.0 - abs(modulo(h * 6.0, 2.0) - 1.0));
    let m = v - c;

    if h < 1.0 / 6.0 {
        return vec3<f32>(c, x, 0.0) + vec3<f32>(m, m, m);
    } else if h < 2.0 / 6.0 {
        return vec3<f32>(x, c, 0.0) + vec3<f32>(m, m, m);
    } else if h < 3.0 / 6.0 {
        return vec3<f32>(0.0, c, x) + vec3<f32>(m, m, m);
    } else if h < 4.0 / 6.0 {
        return vec3<f32>(0.0, x, c) + vec3<f32>(m, m, m);
    } else if h < 5.0 / 6.0 {
        return vec3<f32>(x, 0.0, c) + vec3<f32>(m, m, m);
    } else {
        return vec3<f32>(c, 0.0, x) + vec3<f32>(m, m, m);
    }
}
