use glfw::Window;
use wgpu::{
    Device, Instance, Queue, RenderPipeline, Surface, SurfaceConfiguration, SurfaceError, TextureViewDescriptor
};

use crate::renderer_backend::{mesh_builder, pipeline_builder::PipelineBuilder};

// State manger to handle window rendering
pub struct State<'a> {
    instance: Instance,   //Wgpu Instance
    surface: Surface<'a>, //Where we draw
    device: Device,       // GPU interface
    queue: Queue,         // Commands for the GPU
    config: SurfaceConfiguration,
    pub surface_size: (i32, i32), // window size
    pub window: &'a mut Window,
    render_pipeline: RenderPipeline,
    triangle_mesh: wgpu::Buffer,
}

impl<'a> State<'a> {
    pub async fn new(window: &'a mut Window) -> Self {
        // Surface size has to be the same as the window
        let surface_size = window.get_framebuffer_size();
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());
        let surface = instance.create_surface(window.render_context()).unwrap();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions::default())
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default())
            .await
            .unwrap();

        let surface_capabilities = surface.get_capabilities(&adapter);
        let surface_format = surface_capabilities.formats[0];
        //configure the surface
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT, // We can render textures to the surface
            format: surface_format,
            width: surface_size.0 as u32,
            height: surface_size.1 as u32,
            present_mode: surface_capabilities.present_modes[0],
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        let triangle_mesh = mesh_builder::Vertex::make_triangle(&device);

        let mut pipeline_builder = PipelineBuilder::new();
        pipeline_builder.add_buffer_layout(mesh_builder::Vertex::get_layout());
        pipeline_builder.set_shader_module("shaders/shader.wgsl", "vs_main", "fs_main");
        pipeline_builder.set_pixel_format(config.format);
        let render_pipeline = pipeline_builder.build_pipeline(&device);
        Self {
            instance,
            window,
            surface,
            device,
            queue,
            config,
            surface_size,
            render_pipeline,
            triangle_mesh,
        }
    }
    pub fn render(&mut self) -> Result<(), SurfaceError> {
        let drawable = self.surface.get_current_texture()?;
        let image_view_descriptor = TextureViewDescriptor::default();
        // View of a texture for rendering
        let image_view = drawable.texture.create_view(&image_view_descriptor);

        //Records GPU commands for the frame
        let mut command_encoder: wgpu::CommandEncoder =
            self.device.create_command_encoder(&Default::default());
        // Where and how to render
        let color_attachment = wgpu::RenderPassColorAttachment {
            depth_slice: None,
            view: &image_view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                store: wgpu::StoreOp::Store,
            },
        };
        let render_pass_descriptor = wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(color_attachment)],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        };

        {
            let mut render_pass = command_encoder.begin_render_pass(&render_pass_descriptor);
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_vertex_buffer(0, self.triangle_mesh.slice(..));
            render_pass.draw(0..3, 0..1);
        }
        // commands are sent to GPU
        self.queue.submit(std::iter::once(command_encoder.finish()));
        //Show frame to screen
        drawable.present();

        Ok(())
    }
    pub fn resize(&mut self, new_size: (i32, i32)) {
        if new_size.0 > 0 && new_size.1 > 0 {
            self.surface_size = new_size;
            self.config.width = new_size.0 as u32;
            self.config.height = new_size.1 as u32;
            self.surface.configure(&self.device, &self.config);
        }
    }

    // Recreate surface when it becomes invalid
    pub fn update_surface(&mut self) {
        self.surface = self
            .instance
            .create_surface(self.window.render_context())
            .unwrap();
    }
}
