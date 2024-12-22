use crate::{state::State, utils::load_icon};
use std::sync::Arc;
use winit::{
    application::ApplicationHandler, dpi::PhysicalSize, event::*, event_loop::{ActiveEventLoop, EventLoopProxy}, keyboard::{KeyCode, PhysicalKey}, window::{Window, WindowId}
};

//  Window struct
pub struct App {
    proxy: EventLoopProxy<AppEvent>,
    window: Option<Arc<Window>>,
    state: Option<State>,
    window_id: Option<WindowId>,
}

#[derive(Debug)]
pub enum AppEvent {
    StateReady(State),
}

impl App {
    pub fn new(proxy: EventLoopProxy<AppEvent>) -> Self {
        Self {
            proxy,
            window: None,
            state: None,
            window_id: None,
        }
    }
}

impl ApplicationHandler<AppEvent> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let mut window_attributes = Window::default_attributes();

        
        #[cfg(not(target_arch = "wasm32"))]
        {
            let window_icon: Option<winit::window::Icon> = Some(load_icon("assets/icon.png"));
            window_attributes = window_attributes
                .with_title("Ubik says Learn WGPU")
                .with_window_icon(window_icon);
        }

        #[cfg(target_arch = "wasm32")]
        {
            use winit::platform::web::WindowAttributesExtWebSys;
            window_attributes = window_attributes.with_append(true);
        }


        if let Ok(window) = event_loop.create_window(window_attributes) {
            let first_window_handle = self.window.is_none();
            let window_handle = Arc::new(window);
            self.window_id = Some(window_handle.id());

            State::new(self.proxy.clone(), window_handle.clone());
            self.window = Some(window_handle.clone());
       
        }
    }

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: AppEvent) {
        match event {
            AppEvent::StateReady(state) => self.state = Some(state),
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
                    if let Some(state)= self.state.as_mut() {
                        state.resize(new_size);
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
