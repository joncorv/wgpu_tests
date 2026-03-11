mod wgpu_init;

use std::sync::Arc;
use wgpu_init::WgpuInit;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
};

struct App {
    window: Option<Arc<Window>>,
    state: Option<WgpuInit>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }

        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes().with_title("my app"))
                .unwrap(),
        );

        let state = pollster::block_on(WgpuInit::new(Arc::clone(&window), 1, None));

        let get_info = state.adapter.get_info();
        let get_adapter_limits = state.adapter.limits();
        let get_device_limits = state.device.limits();

        println!("adapter info: {get_info:#?}");
        println!("adapter limits: {get_adapter_limits:#?}");
        println!("device limits: {get_device_limits:#?}");

        self.window = Some(window);
        self.state = Some(state);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(new_size) => {
                if let Some(state) = &mut self.state {
                    state.size = new_size;
                    state.config.width = new_size.width;
                    state.config.height = new_size.height;
                    state.surface.configure(&state.device, &state.config);
                    println!("new sizes: {}, {}", new_size.width, new_size.height);
                }
            }
            WindowEvent::RedrawRequested => {
                // your render logic here
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            _ => {}
        }
    }
}

// async fn get_wgpu_info(window: &Window) {
//     let init = IWgpuInit::new(&window, 1, None);
//
//     let init_move = init.await;
//     let get_info = init_move.adapter.get_info();
//     let get_adapter_limits = init_move.adapter.limits();
//     let get_device_limits = init_move.device.limits();
//
//     println!("adapter info: {get_info:?}");
//     println!("adapter limits: {get_adapter_limits:?}");
//     println!("device limits: {get_device_limits:?}");
// }

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    let mut app = App {
        window: None,
        state: None,
    };
    event_loop.run_app(&mut app).unwrap();
}
