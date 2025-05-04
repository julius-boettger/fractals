pub mod vertex;

use wgpu::util::DeviceExt;
use winit::{
    event::*,
    dpi::PhysicalSize,
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowBuilder},
};

use vertex::{Vertex, VertexFormat};

struct State<'a> {
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: PhysicalSize<u32>,
    window: &'a Window,
    render_pipeline: wgpu::RenderPipeline,
    num_indices: u32,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer, 
    /// whether the window should be redrawn
    redraw: bool,
}

impl<'a> State<'a> {
    async fn new(window: &'a Window, vertices: &Vec<Vertex>, vertex_format: VertexFormat) -> State<'a> {
        let size = window.inner_size();

        // to create surface and adapter
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::VULKAN,
            ..Default::default()
        });

        // to display rendered images
        let surface = instance.create_surface(window).unwrap();

        // handle to chosen gpu
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            // needs to be able to draw on the surface
            compatible_surface: Some(&surface),
            ..Default::default()
        }).await.unwrap();

        // actual gpu device and rendering queue
        let (device, queue) = adapter.request_device(
            &Default::default(),
            None,
        ).await.unwrap();

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

        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));
        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("render pipeline layout"),
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

        let vertices = match vertex_format {
            VertexFormat::Lines => &vertex::lines_as_triangles(&vertices, 0.005),
            VertexFormat::Triangles => vertices,
        };

        let (vertices, indices) = vertex::index(&vertices);

        let num_indices = indices.len().try_into().unwrap();

        // bytemuck cast to slice of bytes
        let vertices = bytemuck::cast_slice(vertices.as_slice());
        let  indices = bytemuck::cast_slice( indices.as_slice());

        if vertices.len() > device.limits().max_buffer_size as usize {
            log::error!("computed vertices are too large to buffer on this device");
            std::process::exit(1);
        }

        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("vertex buffer"),
                usage: wgpu::BufferUsages::VERTEX,
                contents: vertices,
            }
        );

        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("index buffer"),
                usage: wgpu::BufferUsages::INDEX,
                contents: indices,
            }
        );

        let redraw = true;

        Self { surface, device, queue, config, size, window, render_pipeline, num_indices, vertex_buffer, index_buffer, redraw }
    }

    fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            self.redraw = true;
        }
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
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
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        render_pass.draw_indexed(0..self.num_indices, 0, 0..1);

        // free encoder borrow
        drop(render_pass);

        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

pub async fn render(vertices: &Vec<Vertex>, vertex_format: VertexFormat) {
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut state = State::new(&window, vertices, vertex_format).await;

    // only start rendering once surface is configured
    let mut surface_configured = false;

    // if (probably) profiling: exit here before entering the infinite event loop
    if let Ok(value) = std::env::var("CARGO_PROFILE_RELEASE_DEBUG") {
        if value == "true" {
            println!("detected environment variable CARGO_PROFILE_RELEASE_DEBUG=true");
            println!("early-exiting now before entering event loop");
            std::process::exit(0);
        }
    }

    event_loop.run(move |event, control_flow|
        if let Event::WindowEvent { ref event, window_id } = event {
            if window_id == state.window.id() {
                match event {

                    WindowEvent::Resized(physical_size) => {
                        // this also (re)configures the surface 
                        state.resize(*physical_size);
                        surface_configured = true;
                    }

                    WindowEvent::RedrawRequested => {
                        if state.redraw {
                            // tell winit that we want another frame after this one
                            state.window.request_redraw();

                            // dont try to render when surface is not configured yet
                            if !surface_configured {
                                return;
                            }

                            match state.render() {
                                // frame took to long to present
                                Err(wgpu::SurfaceError::Timeout) =>
                                    println!("surface timeout"),

                                Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) =>
                                    state.resize(state.size),

                                Err(wgpu::SurfaceError::OutOfMemory) => {
                                    eprintln!("out of memory");
                                    control_flow.exit();
                                }

                                Err(wgpu::SurfaceError::Other) => {
                                    eprintln!("generic surface error");
                                    control_flow.exit();
                                }

                                Ok(_) => ()
                            }

                            // only redraw once as we are rendering a still image
                            state.redraw = false;
                        }
                    }

                    WindowEvent::CloseRequested => control_flow.exit(),

                    // exit on ESC or Q
                    WindowEvent::KeyboardInput {
                        event: KeyEvent {
                            state: ElementState::Pressed,
                            physical_key: PhysicalKey::Code(KeyCode::Escape | KeyCode::KeyQ),
                            ..
                        },
                        ..
                    } => control_flow.exit(),

                    _ => ()
                }
            }
        }
    ).unwrap();
}
