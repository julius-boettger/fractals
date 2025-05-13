pub mod vertex;

use std::sync::Arc;
use rayon::prelude::*;
use wgpu::util::DeviceExt;
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::{ElementState, KeyEvent, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{Icon, Window, WindowId}
};

use vertex::{Vertex, VertexFormat};
use crate::curves::{Curve, CURVES, INITIAL_ITERATION};

// store icon in executable so we can still distribute just a single file
const ICON_32_BYTES: &[u8] = include_bytes!("../../res/icon/32x32.png");
/// how many seconds an animation cycle should take.
/// must be < 60.
const SECS_PER_ANIMATION_CYCLE: f32 = 5.;

#[repr(C)]
#[derive(Default, Clone, Copy, Debug, bytemuck::Zeroable, bytemuck::Pod)]
// note that types were chosen to correspond to the few available options in WGSL
struct UniformBufferContent {
    /// highest iteration value present in the current vertices
    max_iteration: u32,
    /// smoothly changing value in range [0.0, 1.0)
    animation_value: f32,
}

struct State {
    surface: wgpu::Surface<'static>,
    surface_configured: bool,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    animate: bool,
    animation_value_offset: f32,
    curve_index: usize,
    curve: Box<dyn Curve>,
    iteration: usize,
    size: PhysicalSize<u32>,
    window: Arc<Window>,
    num_indices: u32,
    uniform_buffer_content: UniformBufferContent, 
    vertex_buffer: Option<wgpu::Buffer>,
    index_buffer: Option<wgpu::Buffer>, 
    uniform_buffer: wgpu::Buffer, 
    uniform_buffer_bind_group: wgpu::BindGroup,
    render_pipeline: wgpu::RenderPipeline,
}

impl State {

    const INITIAL_ANIMATE: bool = true;

    async fn new(window: Arc<Window>) -> Self {
        let size = window.inner_size();

        // to create surface and adapter
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::VULKAN,
            ..Default::default()
        });

        // to display rendered images
        let surface = instance.create_surface(window.clone()).unwrap();

        // will be set to true on first resize
        let surface_configured = false;


        // handle to chosen gpu
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            // needs to be able to draw on the surface
            compatible_surface: Some(&surface),
            ..Default::default()
        }).await.unwrap();

        // actual gpu device and rendering queue
        let (device, queue) = adapter.request_device(&Default::default()).await.unwrap();

        let surface_caps = surface.get_capabilities(&adapter);

        // attempt to retrieve a format that uses srgb (standard rgb, 8 bit per channel)
        let surface_format = surface_caps.formats.iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        let animate = Self::INITIAL_ANIMATE;
        let animation_value_offset = 0.;
        let curve_index = 0;
        let curve = CURVES[curve_index]();
        let iteration = INITIAL_ITERATION;
        let num_indices = Default::default();
        let uniform_buffer_content = Default::default();
        let vertex_buffer = None;
        let index_buffer = None;

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("uniform buffer"),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            contents: bytemuck::cast_slice(&[uniform_buffer_content]),
        });
        
        let uniform_buffer_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("uniform buffer bind group layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let uniform_buffer_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("uniform buffer bind group"),
            layout: &uniform_buffer_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));
        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("render pipeline layout"),
            bind_group_layouts: &[&uniform_buffer_bind_group_layout],
            ..Default::default()
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("render pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vertex"),
                buffers: &[Vertex::buffer_layout()],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fragment"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                ..Default::default()
            },
            multisample: Default::default(),
            depth_stencil: None,
            multiview: None,
            cache: None,
        });

        let mut state = Self { surface, surface_configured, device, queue, config, animate, animation_value_offset, curve_index, curve, iteration, size, window, num_indices, uniform_buffer_content, vertex_buffer, index_buffer, uniform_buffer, uniform_buffer_bind_group, render_pipeline };
        state.update_buffers();
        state
    }

    fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            self.surface_configured = true;
        }
    }

    fn set_control_flow(&self, event_loop: &ActiveEventLoop) {
        event_loop.set_control_flow(match self.animate {
            true  => ControlFlow::Poll, // for rendering moving images
            false => ControlFlow::Wait, // for rendering still images
        });
    }

    fn initialize_curve(&mut self) {
        self.curve = CURVES[self.curve_index]();
        self.iteration = INITIAL_ITERATION;
        self.update_buffers();
    }

    fn update_uniform_buffer(&mut self) {
        self.queue.write_buffer(&self.uniform_buffer, 0,
            bytemuck::cast_slice(&[self.uniform_buffer_content]));
    }

    fn update_animation_value(&mut self, set_offset: bool) {
        let seconds = {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap();
    
            // in seconds, in range [0.0, 1.0)
            let nanoseconds = now.subsec_nanos() as f32 * 1e-9;
            // in range [0, 60)
            let seconds = now.as_secs() % 60;

            seconds as f32 + nanoseconds
        };

        // in range [0.0, 1.0)
        let normalized_time = (seconds % SECS_PER_ANIMATION_CYCLE) / SECS_PER_ANIMATION_CYCLE;

        if set_offset {
            self.animation_value_offset = ((self.uniform_buffer_content.animation_value + 1.) - normalized_time) % 1.;
        } else {
            self.uniform_buffer_content.animation_value = (normalized_time + self.animation_value_offset) % 1.;
        }
    }

    fn update_buffers(&mut self) {
        let vertex_format = self.curve.vertex_format();
        let vertices = self.curve.vertices(self.iteration);

        let vertices = match vertex_format {
            VertexFormat::Lines => &vertex::lines_as_triangles(&vertices, 0.005),
            VertexFormat::Triangles => vertices,
        };

        let (vertices, indices) = vertex::index(&vertices);

        self.num_indices = indices.len().try_into().unwrap();

        self.uniform_buffer_content.max_iteration = vertices.par_iter()
            .map(|v| v.iteration)
            .max()
            .unwrap();

        // cast buffer data to slice of bytes
        use bytemuck::cast_slice;
        let vertices = cast_slice(vertices.as_slice());
        let indices = cast_slice(indices.as_slice());

        if vertices.len() > self.device.limits().max_buffer_size as usize {
            log::error!("computed vertices are too large to buffer on this device");
            std::process::exit(1);
        }

        self.vertex_buffer = Some(self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("vertex buffer"),
            usage: wgpu::BufferUsages::VERTEX,
            contents: vertices,
        }));

        self.index_buffer = Some(self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("index buffer"),
            usage: wgpu::BufferUsages::INDEX,
            contents: indices,
        }));


        if self.animate {
            // uniform buffer will be updated later in the loop either way
        } else {
            self.update_uniform_buffer();
        }

        self.window.request_redraw();
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        // wait until surface has been configured
        if !self.surface_configured {
            return Ok(());
        }

        // frame to render to
        let output = self.surface.get_current_texture()?;

        let view = output.texture.create_view(&Default::default());

        // buffer to send commands to the gpu
        let mut encoder = self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor { label: Some("render encoder") }
        );

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("render pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    ..Default::default()
                },
            })],
            ..Default::default()
        });

        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.uniform_buffer_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.as_ref().unwrap().slice(..));
        render_pass.set_index_buffer(self.index_buffer.as_ref().unwrap().slice(..), wgpu::IndexFormat::Uint32);
        render_pass.draw_indexed(0..self.num_indices, 0, 0..1);

        // free encoder borrow
        drop(render_pass);

        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}


#[derive(Default)]
struct App {
    state: Option<State>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let icon = {
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
        macro_rules! key_pressed {
            ($key:ident) => {
                WindowEvent::KeyboardInput {
                    event: KeyEvent {
                        state: ElementState::Pressed,
                        physical_key: PhysicalKey::Code(KeyCode::$key),
                        ..
                    },
                    ..
                }
            };
        }

        let state = self.state.as_mut().unwrap();
        match event {
            key_pressed!(ArrowUp) => {
                state.iteration += 1;
                state.update_buffers();
            },

            key_pressed!(ArrowDown) => {
                if state.iteration > 0 {
                    state.iteration -= 1;
                    state.update_buffers();
                }
            },

            key_pressed!(ArrowLeft) => {
                state.curve_index = if state.curve_index == 0 {
                    CURVES.len() - 1
                } else {
                    state.curve_index - 1
                };
                state.initialize_curve();
            },

            key_pressed!(ArrowRight) => {
                state.curve_index = if state.curve_index == CURVES.len() - 1 {
                    0   
                } else {
                    state.curve_index + 1
                };
                state.initialize_curve();
            },

            key_pressed!(Space) => {
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

                    Ok(_) => ()
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

pub fn render() {
    let event_loop = EventLoop::new().unwrap();
    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}
