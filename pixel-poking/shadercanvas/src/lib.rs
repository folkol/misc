use wgpu::{SurfaceError, TextureFormat};
use winit::event::{Event, WindowEvent};
use winit::event_loop::ControlFlow;
use winit::window::{Window, WindowBuilder};

struct State {
    /// the part of the window that we draw to
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    window: Window,
}

impl State {
    async fn new(window: Window) -> Self {
        let size = window.inner_size();

        // The instance is a handle to our GPU
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(), // Vulkan, Metal, DX12, WebGPU
            dx12_shader_compiler: Default::default(), // Hmm?
        });

        instance
            .enumerate_adapters(wgpu::Backends::all())
            .for_each(|adapter| println!("{adapter:?}"));

        // the surface needs to live as long as the window that created it
        // owned by State, so "should be safe"?
        let surface = unsafe { instance.create_surface(&window).unwrap() };

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        for feature in adapter.features() {
            print!("{feature:?}");
        }

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                    label: None,
                },
                None,
            )
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(TextureFormat::is_srgb)
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width, // Make sure that the width and height of the `SurfaceTexture` are not 0, as that can cause your app to crash.
            height: size.height,
            present_mode: surface_caps.present_modes[0], // When a present command is executed on the gpu, the frame will ...
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        Self {
            window,
            surface,
            device,
            queue,
            config,
            size,
        }
    }
    pub fn window(&self) -> &Window {
        &self.window
    }
    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config)
        }
    }
    fn input(&mut self, event: &WindowEvent) -> bool {
        false
    }
    fn update(&mut self) {}
    fn render(&mut self, r: f64, g: f64, b: f64) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        // Most modern graphics frameworks expect commands to be stored in a command buffer before being sent to the gpu.
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder")
        });
        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r,
                            g,
                            b,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

pub async fn run() {
    let event_loop = winit::event_loop::EventLoop::new();
    let mut input_helper = winit_input_helper::WinitInputHelper::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut state = State::new(window).await;

    event_loop.run(move |event, _, control_flow| {
        if input_helper.update(&event) && input_helper.close_requested() {
            *control_flow = ControlFlow::Exit
        }
        let mut r = 0.1f64;
        let mut g = 0.2f64;
        let b = 0.3f64;
        if let Some((x, y)) = input_helper.mouse() {
            r = x as f64 / state.size.width as f64;
            g = y as f64 / state.size.height as f64;
        }

        // TODO: Needed?
        match event {
            Event::RedrawRequested(window_id) if window_id == state.window().id() => {
                state.update();
                match state.render(r, g, b) {
                    Ok(_) => {}
                    // Reconfigure the surface if lost
                    Err(wgpu::SurfaceError::Lost) => { state.resize(state.size) }
                    Err(wgpu::SurfaceError::OutOfMemory) => {
                        eprintln!("OOM");
                        *control_flow = ControlFlow::ExitWithCode(1);
                    }
                    Err(e) => eprintln!("{e:?}"),
                }
            }
            Event::MainEventsCleared => {
                state.window().request_redraw();
            }
            Event::WindowEvent {
                ref event,
                window_id
            } if window_id == state.window().id() => {
                match event {
                    WindowEvent::Resized(physical_size) => {
                        state.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        state.resize(**new_inner_size);
                    }
                    _ => {}
                };
            }
            _ => {}
        }
    })
}

