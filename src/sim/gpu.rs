use bytemuck::{Pod, Zeroable};
use nannou::prelude::*;

const WORK_GROUP_SIZE: u32 = 256;

#[repr(C)]
#[derive(Default, Clone, Copy, Pod, Zeroable, Debug)]
pub struct Uniforms {
    pub scale: f32,
    pub aspect_ratio: f32,
    pub camera_translation: [f32; 2], // Camera drag translation
    pub window_size: [f32; 2],        // [width, height] in pixels
    pub rotation_angle: f32,          // Rotation angle in radians
    _padding: f32,
    pub rotation_center: [f32; 2], // Point to rotate around (in world coordinates)
}

impl Uniforms {
    pub fn new(scale: f32, camera_translation: Vec2, window_size: Vec2) -> Self {
        Self {
            scale,
            aspect_ratio: window_size.x / window_size.y,
            camera_translation: camera_translation.into(),
            window_size: window_size.into(),
            rotation_angle: 0.0,
            _padding: 0.0,
            rotation_center: [0.0, 0.0],
        }
    }

    pub fn with_rotation(
        scale: f32,
        camera_translation: Vec2,
        window_size: Vec2,
        rotation_angle: f32,
        rotation_center: Vec2,
    ) -> Self {
        Self {
            scale,
            aspect_ratio: window_size.x / window_size.y,
            camera_translation: camera_translation.into(),
            window_size: window_size.into(),
            rotation_angle,
            _padding: 0.0,
            rotation_center: rotation_center.into(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug)]
struct DispatchParams {
    offset: u32,
    dt: f32,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug)]
pub struct GpuAttractor {
    stages: [[f32; 2]; 4],
    // position: [f32; 2],
    mass: f32,
    _padding: f32,
}

impl GpuAttractor {
    pub fn new(stages: [[f32; 2]; 4], mass: f32) -> Self {
        GpuAttractor {
            stages,
            mass,
            _padding: 0.0,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug)]
struct StagePositions([[f32; 2]; 4]);

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug)]
pub struct GpuDust {
    position: [f32; 2],
    velocity: [f32; 2],
}

impl GpuDust {
    pub fn new(position: Vec2, velocity: Vec2) -> Self {
        GpuDust {
            position: [position.x, position.y],
            velocity: [velocity.x, velocity.y],
        }
    }
}

pub(crate) struct DustChunk {
    pub(crate) num_particles: u32,
    pub(crate) compute_bind_group: wgpu::BindGroup,
    pub(crate) render_bind_group: wgpu::BindGroup,
}

#[repr(C)]
pub struct GpuState {
    compute_pipeline: wgpu::ComputePipeline,
    pub(crate) render_pipeline: wgpu::RenderPipeline,
    attractor_buffer: wgpu::Buffer,
    uniform_buffer: wgpu::Buffer,
    dispatch_buffer: wgpu::Buffer,
    attractor_bind_group: wgpu::BindGroup,
    pub(crate) dust_chunks: Vec<DustChunk>,
    pub(crate) uniform_bind_group: wgpu::BindGroup,
    dispatch_bind_group: wgpu::BindGroup,

    pub num_particles: u32,
}

impl GpuState {
    pub fn new(
        device: &wgpu::Device,
        attractors: &[GpuAttractor],
        dust_particles: &[GpuDust],
    ) -> Self {
        let max_buffer_size = device.limits().max_buffer_size as usize;
        let max_storage_binding_size = device.limits().max_storage_buffer_binding_size as usize;
        let dust_stride = std::mem::size_of::<GpuDust>();
        let chunk_byte_limit = max_buffer_size.min(max_storage_binding_size);
        let max_particles_per_chunk = (chunk_byte_limit / dust_stride).max(1);

        let attractor_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Attractor Buffer"),
            contents: bytemuck::cast_slice(attractors),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        });

        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Uniform Buffer"),
            size: std::mem::size_of::<Uniforms>() as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let dispatch_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Dispatch Params Buffer"),
            size: std::mem::size_of::<DispatchParams>() as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Bind groups
        let attractor_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("Attractor Bind Group Layout"),
            });

        let attractor_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &attractor_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: attractor_buffer.as_entire_binding(),
            }],
            label: Some("Attractor Bind Group"),
        });

        let dust_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("Dust Bind Group Layout"),
            });

        let render_dust_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("Render Dust Bind Group Layout"),
            });

        let mut dust_chunks = Vec::new();
        for chunk in dust_particles.chunks(max_particles_per_chunk) {
            let chunk_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Dust Chunk Buffer"),
                contents: bytemuck::cast_slice(chunk),
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::VERTEX,
            });

            let compute_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &dust_bind_group_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: chunk_buffer.as_entire_binding(),
                }],
                label: Some("Dust Chunk Compute Bind Group"),
            });

            let render_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &render_dust_bind_group_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: chunk_buffer.as_entire_binding(),
                }],
                label: Some("Dust Chunk Render Bind Group"),
            });

            dust_chunks.push(DustChunk {
                num_particles: chunk.len() as u32,
                compute_bind_group,
                render_bind_group,
            });
        }

        let uniform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("Uniform Bind Group Layout"),
            });

        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
            label: Some("Uniform Bind Group"),
        });

        let dispatch_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("Dispatch Params Bind Group Layout"),
            });

        let dispatch_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &dispatch_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: dispatch_buffer.as_entire_binding(),
            }],
            label: Some("Dispatch Bind Group"),
        });

        // Load shaders
        let compute_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Compute Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("./shaders/compute.wgsl").into()),
        });

        let render_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Render Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("./shaders/render.wgsl").into()),
        });

        // Create pipeline layouts
        let compute_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Compute Pipeline Layout"),
                bind_group_layouts: &[
                    &dust_bind_group_layout,
                    &attractor_bind_group_layout,
                    &dispatch_bind_group_layout,
                ],
                push_constant_ranges: &[],
            });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&uniform_bind_group_layout, &render_dust_bind_group_layout],
                push_constant_ranges: &[],
            });

        // Create pipelines
        let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("Compute Pipeline"),
            layout: Some(&compute_pipeline_layout),
            module: &compute_shader,
            entry_point: "main",
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &render_shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &render_shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: Frame::TEXTURE_FORMAT,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::SrcAlpha,
                            dst_factor: wgpu::BlendFactor::One,
                            operation: wgpu::BlendOperation::Add,
                        },
                        alpha: wgpu::BlendComponent::REPLACE,
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::PointList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 4,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        GpuState {
            compute_pipeline,
            render_pipeline,
            attractor_buffer,
            uniform_buffer,
            dispatch_buffer,
            attractor_bind_group,
            dust_chunks,
            uniform_bind_group,
            dispatch_bind_group,

            num_particles: dust_particles.len() as u32,
        }
    }

    pub fn update_uniforms(&self, queue: &wgpu::Queue, uniforms: &Uniforms) {
        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::bytes_of(uniforms));
    }

    pub fn update(
        &self,
        dt: f32,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        gpu_attractors: &[GpuAttractor],
    ) {
        queue.write_buffer(
            &self.attractor_buffer,
            0,
            bytemuck::cast_slice(gpu_attractors),
        );
        let mut compute_encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Compute Encoder"),
        });

        for chunk in &self.dust_chunks {
            let num_workgroups = chunk.num_particles.div_ceil(WORK_GROUP_SIZE);
            let params = DispatchParams { offset: 0, dt };
            queue.write_buffer(&self.dispatch_buffer, 0, bytemuck::bytes_of(&params));

            {
                let mut compute_pass =
                    compute_encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                        label: Some("Compute Pass"),
                    });
                compute_pass.set_pipeline(&self.compute_pipeline);
                compute_pass.set_bind_group(0, &chunk.compute_bind_group, &[]);
                compute_pass.set_bind_group(1, &self.attractor_bind_group, &[]);
                compute_pass.set_bind_group(2, &self.dispatch_bind_group, &[]);
                compute_pass.dispatch_workgroups(num_workgroups, 1, 1);
            }
        }

        queue.submit(Some(compute_encoder.finish()));
    }
}
