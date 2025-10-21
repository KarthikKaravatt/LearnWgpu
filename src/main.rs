extern crate glfw;

use glfw::{Action, ClientApiHint, Glfw, Key, WindowEvent, WindowHint, WindowMode};
use glfw::fail_on_errors;
use learn_wgpu::state::State;
use wgpu::SurfaceError;


fn key_events(event: &WindowEvent, window: &mut glfw::Window, glfw: &mut Glfw) {
    match event {
        WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
        WindowEvent::Key(Key::F11, _, Action::Press, _) => {
            glfw.with_primary_monitor(|_, opt_mon| {
                if let Some(primary_monitor) = opt_mon {
                    let primary_mode = primary_monitor.get_video_mode().unwrap();
                    match window.with_window_mode(|mode| match mode {
                        WindowMode::FullScreen(_) => true,
                        WindowMode::Windowed => false,
                    }) {
                        true => window.set_monitor(
                            WindowMode::Windowed,
                            800,
                            600,
                            primary_mode.width,
                            primary_mode.height,
                            Some(primary_mode.refresh_rate),
                        ),
                        false => window.set_monitor(
                            WindowMode::FullScreen(primary_monitor),
                            0,
                            0,
                            primary_mode.width,
                            primary_mode.height,
                            Some(primary_mode.refresh_rate),
                        ),
                    };
                }
            })
        }
        _ => (),
    }
}
fn window_events(event: &WindowEvent, state:&mut State) {
    match event {
        WindowEvent::Pos(..)=>{
            state.update_surface();
            state.resize(state.surface_size);
        },
        WindowEvent::FramebufferSize(width,height)=>{
            state.update_surface();
            state.resize((*width, *height));
        }
        _ =>()
    }
}


async fn run(){
    let mut glfw = glfw::init(fail_on_errors!()).unwrap();
    // Disable OpenGL context (we are using WebGPU)
    glfw.window_hint(WindowHint::ClientApi(ClientApiHint::NoApi));
    let (mut window, events) = glfw
        .create_window(800, 600, "Hello this is window", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    let mut state = State::new(&mut window).await;
    // Event polling so we can react to changes
    state.window.set_framebuffer_size_polling(true);
    state.window.set_key_polling(true);
    state.window.set_mouse_button_polling(true);
    state.window.set_pos_polling(true);

    // Event Loop
    while !state.window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            key_events(&event, &mut state.window, &mut glfw);
            window_events(&event, &mut state);
        }
            match state.render() {
                Ok(_) => {},
                Err(SurfaceError::Lost | SurfaceError::Outdated) => {
                    // Lost is when we minimize the window and Outdated is when the window
                    // resolution changes i.e window resizes
                    state.update_surface();
                    state.resize(state.surface_size);
                },
                Err(e)=> eprintln!("{:?}",e),
            }
    }

}

fn main() {
    pollster::block_on(run());
}
