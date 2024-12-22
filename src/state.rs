use pollster;
use std::sync::Arc;
use wgpu::util::DeviceExt;
use winit::window::Window;
use winit::{dpi::PhysicalSize, event_loop::EventLoopProxy};

use crate::app::AppEvent;
use crate::vertex::{self, Vertex, VERTICES};

// State- Instance
#[derive(Debug)]
pub struct State {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: PhysicalSize<u32>,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    num_vertices: u32,
}

impl State {
    pub async fn new_async(
        proxy: EventLoopProxy<AppEvent>,
        window: Arc<Window>,
        size: PhysicalSize<u32>,
    ) {
        // let size = window.inner_size();

        //>> Instance --------------------

        /*      let instance_descriptor = wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        }; */
                // The instance is a handle to our GPU
        // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU

        let instance_descriptor = wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        };
        let instance = wgpu::Instance::new(instance_descriptor);
        //<<--------------------

        //>> Surface: container of pixels. Where the image is drawn.--------------------
        let surface = instance
            .create_surface(Arc::clone(&window))
            .expect("failed to create a surface");
        //<<--------------------

        //>> Adapter = graphics card --------------------
        let adapter_descriptor = wgpu::RequestAdapterOptionsBase {
            power_preference: wgpu::PowerPreference::default(), // low power/High performance
            compatible_surface: Some(&surface), //  This does not create the surface, only guarantees that the adapter can present to said surface.
            force_fallback_adapter: false, // Indicates that only a fallback adapter can be returned. This is generally a "software" implementation on the system.
        };

        let adapter = instance
            .request_adapter(&adapter_descriptor)
            .await
            .expect("could not get an adapter: i.e graphics card");

        //<<--------------------

        //>>  Device= Logical GPU --------------------

        let device_descriptor = wgpu::DeviceDescriptor {
            required_features: wgpu::Features::empty(), // Specifies the features that are required by the device request. The request will fail if the adapter cannot provide these features.
            required_limits: if cfg!(target_arch = "wasm32") {
                wgpu::Limits::downlevel_webgl2_defaults()
            } else {
                wgpu::Limits::default()
            },
            label: Some("Device"),
            memory_hints: Default::default(),
        };
        let (device, queue) = adapter
            .request_device(&device_descriptor, None)
            .await
            .expect("failed to get a device.");

        //<<--------------------

        //>> Surface: config
        let surface_capabilities = surface.get_capabilities(&adapter);
        let surface_format = surface_capabilities
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_capabilities.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode:   surface_capabilities.present_modes[0], // Fifo is usually supported accross graphic system . Basically a buffer but limit the frame rate  . Mailbox is woth a try
            view_formats: vec![],
            alpha_mode: surface_capabilities.alpha_modes[0],
            desired_maximum_frame_latency: 2, // hint for the number of images to queue up.
        };

        surface.configure(&device, &config);
        //<<--------------------

        //>> Shaders
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/shaders.wgsl").into()),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vertex"),
                buffers: &[Vertex::desc()],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fragment"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(vertex::VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let num_vertices = VERTICES.len() as u32;

        // SEND  new State
        let state = Self {
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            vertex_buffer,
            num_vertices,
        };
        proxy.send_event(AppEvent::StateReady(state)).unwrap();
    }

    pub fn new(proxy: EventLoopProxy<AppEvent>, window: Arc<Window>, size: PhysicalSize<u32>) {
        let future = State::new_async(proxy, window, size);

        #[cfg(not(target_arch = "wasm32"))]
        pollster::block_on(future);

        #[cfg(target_arch = "wasm32")]
        wasm_bindgen_futures::spawn_local(future);
    }

    
    // Window resize
    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    // When rendering, you start a render path.
    // At its core it is a description of the various resources we want to output to.
    // For example, the screen: the color buffer. It is called a color attachment.

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        //  Grab an image to draw to. Swap chain.
        // A swap chain is general structure for queueing images up a to be drawn.
        // You draw and then you swap, it is like a double buffer.
        let drawable = self.surface.get_current_texture()?;
        let image_view_descriptor = wgpu::TextureViewDescriptor::default();
        let image_view = drawable.texture.create_view(&image_view_descriptor);

        // Record all the drawing commands in a struct and submit. Create a command encoder then record command on that and then submit to a queue
        let command_encoder_descriptor = wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        };

        let mut command_encoder = self
            .device
            .create_command_encoder(&command_encoder_descriptor);
        //

        // render path. attachement :  color  buffer

        let color_attachment = wgpu::RenderPassColorAttachment {
            view: &image_view,    // The view to use as an attachment.
            resolve_target: None, //The view that will receive the resolved output  if multisampling texture is used.
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color {
                    // Background color
                    r: 0.8,
                    g: 0.3,
                    b: 0.4,
                    a: 0.9,
                }),
                store: wgpu::StoreOp::Store, // Operation Whether data will be written to through this attachment.
            },
        };
        //   ----

        let render_pass_descriptor = wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(color_attachment)],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        };

        {
            // Command encoder submit and immediately finish
            let mut render_pass = command_encoder.begin_render_pass(&render_pass_descriptor);
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.draw(0..self.num_vertices, 0..1);
        } // <<<< THIS IS IMPORTANT. Drop the

        self.queue.submit(std::iter::once(command_encoder.finish()));

        drawable.present();

        Ok(())
    }
}
