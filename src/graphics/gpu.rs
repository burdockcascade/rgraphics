use std::sync::Arc;
use bytemuck::{Pod, Zeroable};
use cgmath::{Matrix4, Vector3};
use log::{info, trace};
use pollster::FutureExt;
use wgpu::{Adapter, AdapterInfo, BindGroup, Device, Instance, PresentMode, Queue, Surface, SurfaceCapabilities};
use wgpu::core::command::DrawKind::Draw;
use wgpu::util::DeviceExt;
use winit::dpi::PhysicalSize;
use winit::window::Window;
use crate::graphics::draw::{Color, DrawCommand, Mesh};

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
    pub uv: [f32; 2]
}

impl Vertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: size_of::<[f32; 6]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ]
        }
    }
}

pub struct Display {
    surface: Surface<'static>,
    adapter: Adapter,
    device: Device,
    queue: Queue,
    config: wgpu::SurfaceConfiguration,
    render_pipeline: wgpu::RenderPipeline,
    draw_commands: Vec<DrawCommand>,
    size: PhysicalSize<u32>,
    window: Arc<Window>,
}

impl Display {
    pub fn new(window: Window) -> Self {

        let window_arc = Arc::new(window);
        let size = window_arc.inner_size();
        let instance = Self::create_gpu_instance();
        let surface = instance.create_surface(window_arc.clone()).unwrap();
        let adapter = Self::create_adapter(instance, &surface);
        let (device, queue) = Self::create_device(&adapter);
        let surface_caps = surface.get_capabilities(&adapter);
        let config = Self::create_surface_config(size, surface_caps);
        let render_pipeline_layout = Self::create_pipeline_layout(&device);
        let render_pipeline = Self::create_render_pipeline(&device, &render_pipeline_layout, &config);

        surface.configure(&device, &config);

        Self {
            surface,
            adapter,
            device,
            queue,
            config,
            size,
            render_pipeline,
            draw_commands: vec![],
            window: window_arc,
        }
    }

    pub fn set_draw_commands(&mut self, command: Vec<DrawCommand>) {
        self.draw_commands.clear();
        self.draw_commands = command;
    }

    pub fn get_adaptor_info(&self) -> AdapterInfo {
        self.adapter.get_info()
    }

    fn create_surface_config(size: PhysicalSize<u32>, capabilities: SurfaceCapabilities) -> wgpu::SurfaceConfiguration {
        let surface_format = capabilities
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(capabilities.formats[0]);

        wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: PresentMode::AutoNoVsync,
            alpha_mode: capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        }
    }

    fn create_device(adapter: &Adapter) -> (Device, Queue) {
        adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    label: None,
                    memory_hints: Default::default(),
                },
                None,
            )
            .block_on()
            .unwrap()
    }

    fn create_adapter(instance: Instance, surface: &Surface) -> Adapter {
        instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .block_on()
            .unwrap()
    }

    fn create_pipeline_layout(device: &Device) -> wgpu::PipelineLayout {
        let transform_bind_group_layout = Display::create_transform_bind_layout(&device);
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[
                &transform_bind_group_layout,
            ],
            push_constant_ranges: &[],
        })
    }

    fn create_transform_bind_layout(device: &Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
            label: Some("transform_bind_group_layout"),
        })
    }

    fn create_transform_bind_group(device: &Device, transform: [[f32; 4]; 4]) -> BindGroup {

        // 1. Define the BindGroupLayout
        let transform_bind_group_layout = Display::create_transform_bind_layout(&device);

        // 2. Create the buffer to hold the transform data
        let transform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Transform Buffer"),
            contents: bytemuck::cast_slice(&transform),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // 3. Create the BindGroup
        let transform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &transform_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: transform_buffer.as_entire_binding(),
                },
            ],
            label: Some("transform_bind_group"),
        });

        transform_bind_group
    }

    fn create_render_pipeline(device: &Device, layout: &wgpu::PipelineLayout, config: &wgpu::SurfaceConfiguration) -> wgpu::RenderPipeline {

        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent::REPLACE,
                        alpha: wgpu::BlendComponent::REPLACE,
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview: None,
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            cache: None,
        })
    }

    fn create_gpu_instance() -> Instance {
        Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        })
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.size = new_size;

        self.config.width = new_size.width;
        self.config.height = new_size.height;

        self.surface.configure(&self.device, &self.config);

    }

    fn create_left_shift_matrix(shift_amount: f32) -> Matrix4<f32> {
        // Create a translation vector (shift to the left by 'shift_amount')
        let translation = Vector3::new(-shift_amount, 0.0, 0.0);

        // Create the transformation matrix
        Matrix4::from_translation(translation)
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {

        trace!("Start frame");

        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        let background_rgba = Color::WHITE;

        let background_color = wgpu::Color {
            r: background_rgba.r as f64,
            g: background_rgba.g as f64,
            b: background_rgba.b as f64,
            a: background_rgba.a as f64,
        };

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(background_color),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);

            for command in &self.draw_commands {

                // transform bind group
                let bind_group = Display::create_transform_bind_group(&self.device, command.transform.into());
                render_pass.set_bind_group(0, &bind_group, &[]);

                let mesh = &command.mesh;

                let vertex_buffer = mesh.vertex_buffer(&self.device);
                let index_buffer = mesh.index_buffer(&self.device);
                render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
                render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                render_pass.draw_indexed(0..mesh.indices.len() as u32, 0, 0..1);
            }

        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        trace!("End frame");

        Ok(())
    }

    pub fn window(&self) -> &Window {
        &self.window
    }
}