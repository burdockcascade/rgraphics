use std::collections::HashMap;
use crate::graphics::draw::{Color, DrawCommand, Image};
use bytemuck::{Pod, Zeroable};
use image::{DynamicImage, ImageReader, Rgb, RgbaImage};
use log::{debug, info, trace, warn};
use pollster::FutureExt;
use std::sync::Arc;
use wgpu::util::DeviceExt;
use wgpu::{Adapter, AdapterInfo, BindGroup, BindGroupLayout, Buffer, CommandEncoder, Device, Instance, PresentMode, Queue, Surface, SurfaceCapabilities, SurfaceTexture};
use winit::dpi::PhysicalSize;
use winit::window::Window;
use crate::frame::Renderer;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
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
                    format: wgpu::VertexFormat::Float32x2,
                },
            ]
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Uniforms {
    pub transform: [[f32; 4]; 4],
    pub color: [f32; 4],
}

pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
    pub image: RgbaImage
}

impl Texture {

    pub fn from_image(device: &Device, dimg: DynamicImage) -> Self {
        let img = dimg.to_rgba8();

        Texture::new(device, img)
    }

    pub fn new(device: &Device, image: RgbaImage) -> Self {
        let (width, height) = image.dimensions();
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Texture"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: None,
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        Self {
            texture,
            view,
            sampler,
            image
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
    size: PhysicalSize<u32>,
    window: Arc<Window>,
    texture_cache: HashMap<String, Texture>,
    background_color: wgpu::Color
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

        let background_rgba = Color::WHITE;

        let background_color = wgpu::Color {
            r: background_rgba.r as f64,
            g: background_rgba.g as f64,
            b: background_rgba.b as f64,
            a: background_rgba.a as f64,
        };

        Self {
            surface,
            adapter,
            device,
            queue,
            config,
            size,
            render_pipeline,
            window: window_arc,
            texture_cache: HashMap::new(),
            background_color
        }
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
                    label: Some("Device"),
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
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[
                &Display::create_uniform_bind_layout(&device),
                &Display::create_texture_bind_group_layout(&device),
            ],
            push_constant_ranges: &[],
        })
    }

    fn create_uniform_bind_layout(device: &Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
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

    fn create_uniform_bind_group(device: &Device, uniforms: Uniforms) -> BindGroup {
        let uniform_bind_group_layout = Display::create_uniform_bind_layout(&device);
        let transform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniforms Buffer"),
            contents: bytemuck::cast_slice(&[uniforms]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &uniform_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: transform_buffer.as_entire_binding(),
                },
            ],
            label: Some("transform_bind_group"),
        })
    }

    fn create_texture_bind_group_layout(device: &Device) -> BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
            label: Some("texture_bind_group_layout"),
        })
    }

    fn create_texture_bind_group(device: &Device, texture: &Texture) -> BindGroup {
        let texture_bind_group_layout = Display::create_texture_bind_group_layout(&device);
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&texture.sampler),
                }
            ],
            label: Some("texture_bind_group"),
        })
    }

    fn create_render_pipeline(device: &Device, layout: &wgpu::PipelineLayout, config: &wgpu::SurfaceConfiguration) -> wgpu::RenderPipeline {

        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
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
                front_face: wgpu::FrontFace::Cw,
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
        Instance::new(&wgpu::InstanceDescriptor {
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

    fn write_texture_to_queue(queue: &Queue, texture: &Texture) {
        let (width, height) = texture.image.dimensions();
        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &texture.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: Default::default(),
            },
            &texture.image,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * width),
                rows_per_image: Some(height),
            },
            wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
        );
    }
    
    pub fn load_texture(mut self, image: Image) {
        if !self.texture_cache.contains_key(&image.path) {
            let texture = Texture::from_image(&self.device, image.image);
            Display::write_texture_to_queue(&self.queue, &texture);
            self.texture_cache.insert(image.path, texture);
        }
    }
    
    fn create_vertex_buffer(&mut self, vertices: &[Vertex]) -> Buffer {
        self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(vertices),
            usage: wgpu::BufferUsages::VERTEX,
        })
    }

    fn create_index_buffer(&mut self, indices: &[u16]) -> Buffer {
        self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsages::INDEX,
        })
    }
    
    pub fn render(&mut self, renderer: &mut Renderer) {
        
        let output = match self.surface.get_current_texture() {
            Ok(o) => o,
            Err(e) => {
                warn!("Unable to get current texture: {:?}", e);
                return;
            }
        };
        
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
    
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });
    
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(self.background_color),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
    
            render_pass.set_pipeline(&self.render_pipeline);
    
            for command in renderer.commands.iter() {
    
                // uniforms bind group
                let uniforms = Uniforms {
                    transform: command.transform.into(),
                    color: command.color.clone().into(),
                };
                
                let bind_group = Display::create_uniform_bind_group(&self.device, uniforms);
                render_pass.set_bind_group(0, &bind_group, &[]);
    
                let img = &command.image;
                let texture = match self.texture_cache.get(&img.path) {
                    Some(t) => t,
                    None => {
                        // load texture
                        let texture = Texture::from_image(&self.device, img.image.clone());
                        Display::write_texture_to_queue(&self.queue, &texture);
                        self.texture_cache.insert(img.path.clone(), texture);
                        self.texture_cache.get(&img.path).unwrap()
                    }
                };
               
                let bg = Display::create_texture_bind_group(&self.device, &texture);
    
                render_pass.set_bind_group(1, &bg, &[]);
    
                let mesh = &command.mesh;
    
                let vertex_buffer = self.create_vertex_buffer(&mesh.vertices);
                let index_buffer = self.create_index_buffer(&mesh.indices);
                
                render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
                render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                render_pass.draw_indexed(0..mesh.indices.len() as u32, 0, 0..1);
            }
    
        }
    
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        
    }

    pub fn window(&self) -> &Window {
        &self.window
    }
}