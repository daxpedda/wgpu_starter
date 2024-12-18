use crate::{state::State, utils::load_icon};
use std::sync::Arc;
use winit::{
    application::ApplicationHandler, dpi::PhysicalSize, event::*, event_loop::ActiveEventLoop, keyboard::{KeyCode, PhysicalKey}, window::{Window, WindowId}
};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

//  Window struct
#[derive(Default)]
pub struct App<'window> {
    window: Option<Arc<Window>>,
    state: Option<State<'window>>,
    window_id: Option<WindowId>,
    last_size: PhysicalSize<u32>,
}

impl<'window> ApplicationHandler for App<'window> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {

        let mut window_attributes = Window::default_attributes();

        
        #[cfg(not(target_arch = "wasm32"))]
        {
            let window_icon: Option<winit::window::Icon> = Some(load_icon("assets/icon.png"));
            window_attributes = window_attributes
                .with_title("Ubik says Learn WGPU")
                .with_window_icon(window_icon);
        }

        #[allow(unused_assignments)]
        #[cfg(target_arch = "wasm32")]
        let (mut canvas_width, mut canvas_height) = (0, 0);

        #[cfg(target_arch = "wasm32")]
        {
            use winit::platform::web::WindowAttributesExtWebSys;
            let canvas = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .get_element_by_id("canvas")
                .unwrap()
                .dyn_into::<web_sys::HtmlCanvasElement>()
                .unwrap();
            canvas_width = canvas.width();
            canvas_height = canvas.height();
            self.last_size = (canvas_width, canvas_height).into();
            window_attributes = window_attributes.with_canvas(Some(canvas));
        }


        if let Ok(window) = event_loop.create_window(window_attributes) {
            let first_window_handle = self.window.is_none();
            let window_handle = Arc::new(window);
            self.window_id = Some(window_handle.id());

            if first_window_handle {
                #[cfg(not(target_arch = "wasm32"))]
                {
                    let inner_size = window_handle.inner_size();
                    self.last_size = inner_size;
                }    
     
            }
            let state = State::new(window_handle.clone(), self.last_size);
            self.state = Some(state);
            self.window = Some(window_handle.clone());


        
        }

    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let id = self.window.as_ref().unwrap().id();

        if window_id == id {
            match event {
                WindowEvent::CloseRequested => {
                    println!("The close button was pressed; stopping");
                    event_loop.exit();
                }
                WindowEvent::Resized(new_size) => {
                    if let (Some(state), Some(window)) = (self.state.as_mut(), self.window.as_ref())
                    {
                        state.resize(new_size);
                        window.request_redraw();
                        self.last_size = new_size;
                    }
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
                        let _ = state.render();
                    }
                    //self.window.as_ref().unwrap().request_redraw();
                }
                _ => (),
            }
        }
    }
}
