use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    error::EventLoopError,
    event::{ElementState, KeyEvent, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{Icon, Window, WindowId}
};

use super::state::State;
use crate::curves::{Curve, Curves, canopy::Canopy};

#[derive(Default)]
struct App {
    state: Option<State>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let icon = {
            // store icon in executable so we can still distribute just a single file
            const ICON_32_BYTES: &[u8] = include_bytes!("../../res/icon/32x32.png");
            let image = image::load_from_memory(ICON_32_BYTES)
                .unwrap()
                .into_rgba8();
            let (width, height) = image.dimensions();
            Icon::from_rgba(image.into_raw(), width, height).unwrap()
        };

        let window = Arc::new(
            event_loop.create_window(
                Window::default_attributes()
                    .with_title("Fractals")
                    .with_window_icon(Some(icon))
            ).unwrap()
        );

        let state = pollster::block_on(State::new(window.clone()));

        state.set_control_flow(event_loop);

        // if (probably) profiling: exit here before entering the infinite event loop
        if let Ok(value) = std::env::var("CARGO_PROFILE_RELEASE_DEBUG") {
            if value == "true" {
                log::info!("detected environment variable CARGO_PROFILE_RELEASE_DEBUG=true");
                log::info!("early-exiting now before entering event loop");
                std::process::exit(0);
            }
        }

        window.request_redraw();

        self.state = Some(state);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        #[allow(clippy::enum_glob_use)]
        use KeyCode::*;

        let state = self.state.as_mut().unwrap();
        match event {
            WindowEvent::KeyboardInput {
                event: KeyEvent {
                    state: ElementState::Pressed,
                    physical_key: PhysicalKey::Code(key),
                    ..
                },
                ..
            } => match key {
                ArrowUp => {
                    state.iteration += 1;
                    state.update_buffers();
                },
                ArrowDown => {
                    if state.iteration > 0 {
                        state.iteration -= 1;
                        state.update_buffers();
                    }
                },

                ArrowLeft => {
                    state.curve.prev();
                    state.initialize_curve();
                },
                ArrowRight => {
                    state.curve.next();
                    state.initialize_curve();
                },

                key @ (KeyF | KeyD | KeyJ | KeyK) => {
                    if state.curve != Curves::Canopy {
                        return;
                    }

                    let (increment, left) = match key {
                        KeyF => ( true,  true),
                        KeyD => (false,  true),
                        KeyJ => ( true, false),
                        KeyK => (false, false),
                        _ => panic!(),
                    };

                    if Canopy::downcast(&mut state.curve_instance)
                        .change_angle(increment, left)
                    {
                        state.redo_curve();
                    }
                },

                F11 => {
                    state.window.set_fullscreen(match state.window.fullscreen() {
                        None => Some(winit::window::Fullscreen::Borderless(None)),
                        Some(_) => None,
                    });
                },

                Space => {
                    if state.animate {
                        state.animate = false;
                    } else {
                        state.animate = true;
                        state.update_animation_value(true);
                        // to jump-start constantly rendering new frames again
                        state.window.request_redraw();
                    }

                    state.set_control_flow(event_loop);
                },

                _ => ()
            },

            WindowEvent::RedrawRequested => {
                if state.animate {
                    state.update_animation_value(false);
                    state.update_uniform_buffer();
                }

                match state.render() {
                    Err(wgpu::SurfaceError::Timeout) =>
                        log::warn!("surface timeout (frame took too long to present)"),

                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) =>
                        state.resize(state.size),

                    Err(wgpu::SurfaceError::OutOfMemory) => {
                        log::error!("out of memory");
                        event_loop.exit();
                    }

                    Err(wgpu::SurfaceError::Other) => {
                        log::error!("generic surface error");
                        event_loop.exit();
                    }

                    Ok(()) => ()
                }

                if state.animate {
                    // tell winit that we immediately want another frame after this one,
                    // as we are rendering a moving image
                    state.window.request_redraw();
                }
            }

            WindowEvent::Resized(physical_size) => {
                // this also (re)configures the surface 
                state.resize(physical_size);
            }

            WindowEvent::CloseRequested => event_loop.exit(),

            _ => (),
        }
    }
}

pub fn run_app() {
    let event_loop = match EventLoop::new() {
        Ok(event_loop) => event_loop,

        #[cfg(target_os = "linux")]
        Err(EventLoopError::Os(e)) => {
            const ERROR_TABLE: &[(&str, &str)] = &[
                ("WaylandError(Connection(NoWaylandLib))", "wayland libraries (libwayland-client.so/-cursor.so/-egl.so) not found, consider installing them."),
                ("WaylandError(Connection(NoCompositor))", "no running wayland compositor found.\nthis may be caused by an unusual setup which winit (https://docs.rs/winit) does not understand."),
                ("libXi.so", "xorg library libXi.so not found, consider installing it."),
                ("libX11.so", "xorg library libX11.so not found, consider installing it."),
                ("libXcursor.so", "xorg library libXcursor.so not found, consider installing it."),
            ];

            // cant use pattern matching because error types are private,
            // best we can do is match the formatted error string
            let error_string = format!("{e:?}");

            for error_desc in ERROR_TABLE {
                if error_string.contains(error_desc.0) {
                    log::error!("{}", error_desc.1);
                    std::process::exit(1)
                }
            }

            panic!("{e:?}");
        },

        Err(e) => panic!("{e:?}")
    };

    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}
