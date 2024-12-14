use std::iter;
use std::sync::Arc;
use pollster;
use winit::{
    application::ApplicationHandler, dpi::PhysicalSize, error::EventLoopError, event::*, event_loop::{ActiveEventLoop, ControlFlow, EventLoop}, keyboard::{KeyCode, PhysicalKey}, window::{Window, WindowId}
};


//  Window struct
#[derive(Default)]
struct App<'window> {
    window: Option<Arc<Window>>,
    state: Option<State<'window>>,
    window_id: Option<WindowId>,
}

impl<'window> ApplicationHandler for App<'window> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes()
                .with_title("Ubik says Learn WGPU");

        
        let window = Arc::new( event_loop.create_window(window_attributes).expect("create window error"));
        self.window_id = Some(window.id());
        let state = State::new(window.clone());
        self.state = Some(state);
        self.window = Some(window.clone());
  
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {

    let id = self.window.as_ref().unwrap().id();
 

        if window_id == id {
            match event {
                        WindowEvent::CloseRequested => {
                            println!("The close button was pressed; stopping");
                            event_loop.exit();
                        }
                        WindowEvent::RedrawRequested => {
                            // Redraw the application.
                            //
                            // It's preferable for applications that do not render continuously to render in
                            // this event rather than in AboutToWait, since rendering in here allows
                            // the program to gracefully handle redraws requested by the OS.

                            // Draw.
                            // Queue a RedrawRequested event.
                            //
                            // You only need to call this if you've determined that you need to redraw in
                            // applications which do not always need to. Applications that redraw continuously
                            // can render here instead.

                            if let Some(state) = self.state.as_mut() {
                                state.render();
                            }
                            //self.window.as_ref().unwrap().request_redraw();
                        }
                        _ => (),
            }
    
        }
    }
}


// State- Instance
struct State<'window> {
    surface:   wgpu::Surface<'window>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: PhysicalSize<u32>,
}

impl<'window> State<'window> {

    async fn new_async(window: Arc<Window>) -> State<'window> {

        let size = window.inner_size();

        //>> Instance --------------------
        let instance_descriptor = wgpu::InstanceDescriptor{
            backends: wgpu::Backends::all(), ..Default::default()
        };

        let instance = wgpu::Instance::new(instance_descriptor);
        //<<--------------------

        //>> Surface: container of pixels. Where the image is drawn.--------------------
        let surface = instance.create_surface(Arc::clone(&window)).expect("failed to create a surface");
        //<<--------------------

        //>> Adapter = graphics card --------------------
        let adapter_descriptor = wgpu::RequestAdapterOptionsBase{
            power_preference: wgpu::PowerPreference::default(), // low power/High performance
            compatible_surface: Some(&surface), //  This does not create the surface, only guarantees that the adapter can present to said surface.
            force_fallback_adapter: false, // Indicates that only a fallback adapter can be returned. This is generally a "software" implementation on the system.
        };

        let adapter = instance.request_adapter(&adapter_descriptor).await.expect("could not get an adapter: i.e graphics card");

        //<<--------------------

        //>>  Device= Logical GPU --------------------

        let device_descriptor = wgpu::DeviceDescriptor {
            required_features: wgpu::Features::empty(), // Specifies the features that are required by the device request. The request will fail if the adapter cannot provide these features.
            required_limits:  if cfg!(target_arch = "wasm32") {
                wgpu::Limits::downlevel_webgl2_defaults()
            } else {
                wgpu::Limits::default()
            },
            label: Some("Device"),
            memory_hints: Default::default(),
        };
        let (device, queue) = adapter
            .request_device(&device_descriptor, None).await.expect("failed to get a device.");

        //<<--------------------

        //>> Surface: config
        let surface_capabilities = surface.get_capabilities(&adapter);
        let surface_format = surface_capabilities
            .formats
            .iter()
            .copied()
            .filter(|f| f.is_srgb())
            .next()
            .unwrap_or(surface_capabilities.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_capabilities.present_modes[0], // Fifo is usually supported accross graphic system . Basically a buffer but limit the frame rate  . Mailbox is woth a try
            view_formats: vec![],
            alpha_mode: surface_capabilities.alpha_modes[0],
            desired_maximum_frame_latency: 2 , // hint for the number of images to queue up.
            
        };

        surface.configure(&device, &config);
        //<<--------------------

        // RETURN  new State
        Self {
            surface,
            device,
            queue,
            config,
            size,
        }
           
    }
    pub fn new(window: Arc<Window>) -> State<'window> {
        pollster::block_on(State::new_async(window))
    }

    // Window resize
    fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width>0 && new_size.height >0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }


    // When rendering, it starts a render path.
    // At its core it is a description of the various resources we want to output to.
    // For example, the screen: the color buffer. It is called a color attachment.
    
    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {

        //  Grab an image to draw to??
        // A swap chain is general structure for queueing images up a to be drawn.
        // You draw and then you swap, it is like a double buffer.
        let drawable = self.surface.get_current_texture()?;
        let image_view_descriptor = wgpu::TextureViewDescriptor::default();
        let image_view = drawable.texture.create_view(&image_view_descriptor);

        // Record all the drawing commands in a struct and submit. Create a command encoder then record command on that and then submit to a queue
        let command_encoder_descriptor =  wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder")
        };

        let mut command_encoder = self.device.create_command_encoder((&command_encoder_descriptor));
        //

        // render path. attachement :  color  buffer

        let color_attachment  = wgpu::RenderPassColorAttachment {
            view: &image_view,  // The view to use as an attachment.
            resolve_target: None, //The view that will receive the resolved output  if multisampling texture is used. 
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color{ // Background color
                    r: 0.8,
                    g:0.3,
                    b:0.7,
                    a:0.5,
                }),
                store: wgpu::StoreOp::Store // Operation Whether data will be written to through this attachment.
            }
        };
        //   ----

        let render_pass_descriptor = wgpu::RenderPassDescriptor{
            label: Some("Render Pass"),
            color_attachments: &[Some(color_attachment)],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        };

        // Command encoder submit and immediately finish
        command_encoder.begin_render_pass(&render_pass_descriptor);
        self.queue.submit(std::iter::once(command_encoder.finish()));

        drawable.present();

        Ok(())

    }


}



fn main() -> Result<(), EventLoopError> {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
    // dispatched any events. This is ideal for games and similar applications.
    event_loop.set_control_flow(ControlFlow::Poll);

    // ControlFlow::Wait pauses the event loop if no events are available to process.
    // This is ideal for non-game applications that only update in response to user
    // input, and uses significantly less power/CPU time than ControlFlow::Poll.
    //event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = App::default();
    event_loop.run_app(&mut app)

    //let mut state = State::new(&app);
}
