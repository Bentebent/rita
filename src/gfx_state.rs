use std::sync::Arc;

use winit::window::Window;

use crate::render_pipeline::RenderPipelineBuilder;

//We use 'static here because the surface will live for the lifetime of the program
//and because surface has a self-referential reference to window
pub struct GFXState<'a> {
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface_config: wgpu::SurfaceConfiguration,

    render_pipeline: crate::render_pipeline::RenderPipeline,
}

impl<'a> GFXState<'a> {
    pub async fn new(window: Arc<Window>) -> Self {
        let size = window.inner_size();

        // The instance is a handle to our GPU
        // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
        //TODO: support configuring backends
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            #[cfg(not(target_arch = "wasm32"))]
            backends: wgpu::Backends::PRIMARY,
            #[cfg(target_arch = "wasm32")]
            backends: wgpu::Backends::GL,
            ..Default::default()
        });

        let surface = instance
            .create_surface(window.clone())
            .expect("Failed to create wgpu surface");

        //Adapter is our physical graphics card handle
        //TODO: enumerate adapters to select the correct one depending on required hardware support for non WASM
        // targets
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .expect("Failed to find an appropriate adapter");

        //TODO: enumerate available features
        //TODO: support configuring what limits to use
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    required_features: wgpu::Features::empty(),
                    required_limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                    label: None,
                    memory_hints: Default::default(),
                },
                None,
            )
            .await
            .expect("Failed to create device and queue");

        //TODO: support other surface formats
        let surface_capabilites = surface.get_capabilities(&adapter);
        let surface_format = surface_capabilites
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_capabilites.formats[0]);

        //TODO: Implement configuration for all of this
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_capabilites.present_modes[0],
            alpha_mode: surface_capabilites.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        //TODO: refactor shader loading
        let render_pipeline = RenderPipelineBuilder::new("Placeholder pipeline")
            .vertex_state(
                &device.create_shader_module(wgpu::ShaderModuleDescriptor {
                    label: Some("Placeholder vertex shader"),
                    source: wgpu::ShaderSource::Wgsl(include_str!("../data/shaders/tutorial_3.wgsl").into()),
                }),
                "vs_main",
                &[],
            )
            .primitive_state(wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            })
            .multisample_state(wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            })
            .fragment_state(
                &device.create_shader_module(wgpu::ShaderModuleDescriptor {
                    label: Some("Placeholder fragment shader"),
                    source: wgpu::ShaderSource::Wgsl(include_str!("../data/shaders/tutorial_3.wgsl").into()),
                }),
                "fs_main",
                &[Some(wgpu::ColorTargetState {
                    format: surface_config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            )
            .layout(&device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Placeholder pipeline layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            }))
            .build(&device);

        Self {
            surface,
            device,
            queue,
            surface_config,
            render_pipeline,
        }
    }

    pub fn resize(&mut self, new_size: &winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.surface_config.width = new_size.width;
            self.surface_config.height = new_size.height;
            self.surface.configure(&self.device, &self.surface_config)
        }
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        //TODO: replace with proper render pass
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Placeholder encoder"),
        });
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Placeholder render pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 100.0 / 255.0,
                            g: 149.0 / 255.0,
                            b: 237.0 / 255.0,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_pipeline(self.render_pipeline.render_pipeline());
            render_pass.draw(0..3, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
