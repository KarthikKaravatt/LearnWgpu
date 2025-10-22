use std::{env::current_dir, fs::read_to_string};

use wgpu::{
    PipelineCompilationOptions, PipelineLayoutDescriptor, RenderPipelineDescriptor,
    ShaderModuleDescriptor, VertexBufferLayout,
};

pub struct PipelineBuilder {
    shader_filename: String,
    // Entries are the names of the functions in Wgsl
    vertex_entry: String,
    fragment_entry: String,
    pixel_format: wgpu::TextureFormat,
    vertex_buffer_layouts: Vec<wgpu::VertexBufferLayout<'static>>,
}

// The render pipeline describes how something should be drawn on the canvas
impl PipelineBuilder {
    pub fn new() -> Self {
        PipelineBuilder {
            shader_filename: "dummy".to_string(),
            vertex_entry: "dummy".to_string(),
            fragment_entry: "dummy".to_string(),
            pixel_format: wgpu::TextureFormat::Rgba8Unorm,
            vertex_buffer_layouts: Vec::new(),
        }
    }
    pub fn add_buffer_layout(&mut self, layout:VertexBufferLayout<'static>){
        self.vertex_buffer_layouts.push(layout);
    }
    // Configure shader properties of the pipeline
    pub fn set_shader_module(
        &mut self,
        shader_filename: &str,
        vertex_entry: &str,
        fragment_entry: &str,
    ) {
        self.shader_filename = shader_filename.to_string();
        self.vertex_entry = vertex_entry.to_string();
        self.fragment_entry = fragment_entry.to_string();
    }
    pub fn set_pixel_format(&mut self, pixel_format: wgpu::TextureFormat) {
        self.pixel_format = pixel_format;
    }

    pub fn build_pipeline(&mut self, device: &wgpu::Device) -> wgpu::RenderPipeline {
        // load shader from file
        let mut file_path = current_dir().unwrap();
        file_path.push("src/");
        file_path.push(self.shader_filename.as_str());
        let file_path = file_path.into_os_string().into_string().unwrap();
        println!("{file_path}");
        let source_code = read_to_string(file_path).expect("Can't read source code!");

        // Convert shader code to IR that can be read by the GPU
        let shader_module_descriptor = ShaderModuleDescriptor {
            label: Some("Shader Module"),
            source: wgpu::ShaderSource::Wgsl(source_code.into()),
        };
        let shader_module = device.create_shader_module(shader_module_descriptor);

        // Interface between the pipeline and external resources
        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor::default());

        // Describes what happens when the fragment shader outputs a colour
        // New pixels will replace old ones
        // It can write to all colour channels
        let render_tarets = [Some(wgpu::ColorTargetState {
            format: self.pixel_format,
            blend: Some(wgpu::BlendState::REPLACE),
            write_mask: wgpu::ColorWrites::ALL,
        })];

        // Put everything together
        let render_pipeline_descriptor = RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader_module,
                entry_point: Some(&self.vertex_entry),
                compilation_options: PipelineCompilationOptions::default(),
                buffers: &self.vertex_buffer_layouts,
            },
            // How to interpret the vertices
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader_module,
                entry_point: Some(&self.fragment_entry),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                targets: &render_tarets,
            }),
            multiview: None,
            cache: None,
        };
        // GPU creates the pipeline object
        device.create_render_pipeline(&render_pipeline_descriptor)
    }
}
